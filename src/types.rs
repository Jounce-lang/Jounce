// Jounce Type System
// Defines the type representation and type operations

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // Primitive types
    Int,
    Float,
    String,
    Bool,
    Void,
    Any,

    // Component types
    Component(Vec<Type>), // Component with prop types

    // Function types
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },

    // Collection types
    Array(Box<Type>),
    Tuple(Vec<Type>),

    // Generic types
    Generic(String), // Generic type variable (e.g., T, U)

    // Union types
    Union(Vec<Type>),

    // Optional type
    Option(Box<Type>),

    // Type variable (for inference)
    Var(usize),

    // Custom types
    Named(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Void => write!(f, "void"),
            Type::Any => write!(f, "any"),
            Type::Component(props) => {
                write!(f, "Component<")?;
                for (i, prop) in props.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", prop)?;
                }
                write!(f, ">")
            }
            Type::Function { params, return_type } => {
                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::Array(inner) => write!(f, "{}[]", inner),
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", ty)?;
                }
                write!(f, ")")
            }
            Type::Generic(name) => write!(f, "{}", name),
            Type::Union(types) => {
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", ty)?;
                }
                Ok(())
            }
            Type::Option(inner) => write!(f, "{}?", inner),
            Type::Var(id) => write!(f, "τ{}", id),
            Type::Named(name) => write!(f, "{}", name),
        }
    }
}

impl Type {
    /// Check if this type is a primitive type
    pub fn is_primitive(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::String | Type::Bool)
    }

    /// Check if this type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::Any)
    }

    /// Check if two types are compatible (can be assigned)
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            // Any type is compatible with everything
            (Type::Any, _) | (_, Type::Any) => true,

            // Numbers are inter-compatible
            (Type::Int, Type::Float) | (Type::Float, Type::Int) => true,

            // Optional types
            (Type::Option(inner), ty) | (ty, Type::Option(inner)) => {
                inner.as_ref().is_compatible_with(ty)
            }

            // Union types
            (Type::Union(types), ty) => types.iter().any(|t| t.is_compatible_with(ty)),
            (ty, Type::Union(types)) => types.iter().any(|t| ty.is_compatible_with(t)),

            // Arrays
            (Type::Array(a), Type::Array(b)) => a.is_compatible_with(b),

            _ => false,
        }
    }

    /// Get the return type of a function, if this is a function type
    pub fn get_return_type(&self) -> Option<&Type> {
        match self {
            Type::Function { return_type, .. } => Some(return_type),
            _ => None,
        }
    }

    /// Get the parameter types of a function, if this is a function type
    pub fn get_param_types(&self) -> Option<&Vec<Type>> {
        match self {
            Type::Function { params, .. } => Some(params),
            _ => None,
        }
    }

    /// Create a function type
    pub fn function(params: Vec<Type>, return_type: Type) -> Self {
        Type::Function {
            params,
            return_type: Box::new(return_type),
        }
    }

    /// Create an array type
    pub fn array(element_type: Type) -> Self {
        Type::Array(Box::new(element_type))
    }

    /// Create an optional type
    pub fn optional(inner_type: Type) -> Self {
        Type::Option(Box::new(inner_type))
    }

    /// Get free type variables in a type
    pub fn free_vars(&self) -> HashSet<usize> {
        match self {
            Type::Var(id) => {
                let mut set = HashSet::new();
                set.insert(*id);
                set
            }
            Type::Array(inner) => inner.free_vars(),
            Type::Option(inner) => inner.free_vars(),
            Type::Function { params, return_type } => {
                let mut set = HashSet::new();
                for param in params {
                    set.extend(param.free_vars());
                }
                set.extend(return_type.free_vars());
                set
            }
            Type::Tuple(types) => {
                let mut set = HashSet::new();
                for ty in types {
                    set.extend(ty.free_vars());
                }
                set
            }
            Type::Union(types) => {
                let mut set = HashSet::new();
                for ty in types {
                    set.extend(ty.free_vars());
                }
                set
            }
            _ => HashSet::new(),
        }
    }
}

/// Type environment for tracking variable types in scopes
#[derive(Debug, Clone)]
pub struct TypeEnv {
    scopes: Vec<HashMap<String, TypeScheme>>,
    next_var_id: usize,
}

impl TypeEnv {
    pub fn new() -> Self {
        TypeEnv {
            scopes: vec![HashMap::new()],
            next_var_id: 0,
        }
    }

    /// Enter a new scope
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exit the current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Add a variable binding in the current scope (monomorphic)
    pub fn bind(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, TypeScheme::monomorphic(ty));
        }
    }

    /// Add a polymorphic variable binding in the current scope
    pub fn bind_scheme(&mut self, name: String, scheme: TypeScheme) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, scheme);
        }
    }

    /// Look up a variable's type scheme
    pub fn lookup_scheme(&self, name: &str) -> Option<&TypeScheme> {
        for scope in self.scopes.iter().rev() {
            if let Some(scheme) = scope.get(name) {
                return Some(scheme);
            }
        }
        None
    }

    /// Look up a variable's type (for backwards compatibility)
    pub fn lookup(&self, name: &str) -> Option<Type> {
        self.lookup_scheme(name).map(|scheme| {
            // For monomorphic types, just return the type
            // For polymorphic types, return the type without instantiation
            // (caller should use instantiate if needed)
            scheme.ty.clone()
        })
    }

    /// Generalize a type into a type scheme
    /// Quantifies all free type variables in the type that are not free in the environment
    pub fn generalize(&self, ty: &Type) -> TypeScheme {
        let type_free_vars = ty.free_vars();
        let env_free_vars = self.free_vars();

        // Variables to quantify are those free in the type but not in the environment
        let quantified: HashSet<usize> = type_free_vars
            .difference(&env_free_vars)
            .copied()
            .collect();

        TypeScheme::new(quantified, ty.clone())
    }

    /// Get all free type variables in the environment
    pub fn free_vars(&self) -> HashSet<usize> {
        let mut vars = HashSet::new();
        for scope in &self.scopes {
            for scheme in scope.values() {
                vars.extend(scheme.free_vars());
            }
        }
        vars
    }

    /// Generate a fresh type variable
    pub fn fresh_var(&mut self) -> Type {
        let id = self.next_var_id;
        self.next_var_id += 1;
        Type::Var(id)
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}

/// Substitution for type variables
#[derive(Debug, Clone)]
pub struct Substitution {
    map: HashMap<usize, Type>,
}

impl Substitution {
    pub fn new() -> Self {
        Substitution {
            map: HashMap::new(),
        }
    }

    /// Add a substitution from type variable to type
    pub fn insert(&mut self, var: usize, ty: Type) {
        self.map.insert(var, ty);
    }

    /// Apply substitution to a type
    pub fn apply(&self, ty: &Type) -> Type {
        self.apply_with_depth(ty, 0)
    }

    /// Apply substitution with depth limit to prevent infinite recursion
    fn apply_with_depth(&self, ty: &Type, depth: usize) -> Type {
        // Prevent infinite recursion with a depth limit
        if depth > 100 {
            return ty.clone();
        }

        match ty {
            Type::Var(id) => {
                if let Some(substituted) = self.map.get(id) {
                    // Only recursively apply if it's not the same variable (to prevent cycles)
                    if let Type::Var(sub_id) = substituted {
                        if sub_id == id {
                            return ty.clone();
                        }
                    }
                    // Recursively apply substitution with incremented depth
                    self.apply_with_depth(substituted, depth + 1)
                } else {
                    ty.clone()
                }
            }
            Type::Array(inner) => Type::Array(Box::new(self.apply_with_depth(inner, depth + 1))),
            Type::Option(inner) => Type::Option(Box::new(self.apply_with_depth(inner, depth + 1))),
            Type::Function { params, return_type } => Type::Function {
                params: params.iter().map(|p| self.apply_with_depth(p, depth + 1)).collect(),
                return_type: Box::new(self.apply_with_depth(return_type, depth + 1)),
            },
            Type::Tuple(types) => Type::Tuple(types.iter().map(|t| self.apply_with_depth(t, depth + 1)).collect()),
            Type::Union(types) => Type::Union(types.iter().map(|t| self.apply_with_depth(t, depth + 1)).collect()),
            _ => ty.clone(),
        }
    }

    /// Compose two substitutions
    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = Substitution::new();

        // Apply other to all types in self
        for (var, ty) in &self.map {
            result.insert(*var, other.apply(ty));
        }

        // Add mappings from other that aren't in self
        for (var, ty) in &other.map {
            if !result.map.contains_key(var) {
                result.insert(*var, ty.clone());
            }
        }

        result
    }
}

impl Default for Substitution {
    fn default() -> Self {
        Self::new()
    }
}

/// Type scheme for let-polymorphism
/// Represents a polymorphic type like ∀α.α → α
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScheme {
    /// Quantified type variables (the "forall" variables)
    pub quantified: HashSet<usize>,
    /// The actual type
    pub ty: Type,
}

impl TypeScheme {
    /// Create a new type scheme
    pub fn new(quantified: HashSet<usize>, ty: Type) -> Self {
        TypeScheme { quantified, ty }
    }

    /// Create a monomorphic type scheme (no quantified variables)
    pub fn monomorphic(ty: Type) -> Self {
        TypeScheme {
            quantified: HashSet::new(),
            ty,
        }
    }

    /// Instantiate a type scheme by replacing quantified variables with fresh ones
    pub fn instantiate(&self, env: &mut TypeEnv) -> Type {
        if self.quantified.is_empty() {
            // No quantified variables, just return the type
            return self.ty.clone();
        }

        let mut subst = Substitution::new();

        // Create fresh type variables for each quantified variable
        for &var in &self.quantified {
            let fresh = env.fresh_var();
            // Only insert if not already present to avoid cycles
            if !subst.map.contains_key(&var) {
                subst.insert(var, fresh);
            }
        }

        // Apply the substitution to the type
        subst.apply(&self.ty)
    }

    /// Get free type variables in this type scheme (not quantified)
    pub fn free_vars(&self) -> HashSet<usize> {
        let type_vars = self.ty.free_vars();
        type_vars.difference(&self.quantified).copied().collect()
    }
}

impl fmt::Display for TypeScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.quantified.is_empty() {
            write!(f, "{}", self.ty)
        } else {
            write!(f, "∀")?;
            let mut vars: Vec<_> = self.quantified.iter().collect();
            vars.sort();
            for (i, var) in vars.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                write!(f, "τ{}", var)?;
            }
            write!(f, ".{}", self.ty)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_compatibility() {
        assert!(Type::Int.is_compatible_with(&Type::Int));
        assert!(Type::Int.is_compatible_with(&Type::Float));
        assert!(Type::Any.is_compatible_with(&Type::String));
        assert!(Type::Bool.is_compatible_with(&Type::Any));
    }

    #[test]
    fn test_type_env() {
        let mut env = TypeEnv::new();
        env.bind("x".to_string(), Type::Int);
        assert_eq!(env.lookup("x"), Some(Type::Int));

        env.push_scope();
        env.bind("y".to_string(), Type::String);
        assert_eq!(env.lookup("y"), Some(Type::String));
        assert_eq!(env.lookup("x"), Some(Type::Int));

        env.pop_scope();
        assert_eq!(env.lookup("y"), None);
        assert_eq!(env.lookup("x"), Some(Type::Int));
    }

    #[test]
    fn test_substitution() {
        let mut subst = Substitution::new();
        subst.insert(0, Type::Int);

        let ty = Type::Array(Box::new(Type::Var(0)));
        let result = subst.apply(&ty);
        assert_eq!(result, Type::Array(Box::new(Type::Int)));
    }

    #[test]
    fn test_type_scheme_instantiation() {
        let mut env = TypeEnv::new();

        // Create a polymorphic identity function: ∀α.α → α
        let mut quantified = HashSet::new();
        quantified.insert(0);
        let poly_type = Type::function(vec![Type::Var(0)], Type::Var(0));
        let scheme = TypeScheme::new(quantified, poly_type);

        // Instantiate it twice - should get different type variables
        let instance1 = scheme.instantiate(&mut env);
        let instance2 = scheme.instantiate(&mut env);

        // Both should be functions
        assert!(matches!(instance1, Type::Function { .. }));
        assert!(matches!(instance2, Type::Function { .. }));

        // But they should have different type variables
        assert_ne!(instance1, instance2);
    }

    #[test]
    fn test_type_free_vars() {
        // Type with free variables: τ0 → τ1
        let ty = Type::function(vec![Type::Var(0)], Type::Var(1));
        let free_vars = ty.free_vars();

        assert!(free_vars.contains(&0));
        assert!(free_vars.contains(&1));
        assert_eq!(free_vars.len(), 2);
    }

    #[test]
    fn test_type_scheme_free_vars() {
        // Type scheme: ∀τ0.τ0 → τ1
        // Only τ1 is free (τ0 is quantified)
        let mut quantified = HashSet::new();
        quantified.insert(0);
        let ty = Type::function(vec![Type::Var(0)], Type::Var(1));
        let scheme = TypeScheme::new(quantified, ty);

        let free_vars = scheme.free_vars();
        assert!(!free_vars.contains(&0)); // τ0 is quantified
        assert!(free_vars.contains(&1));  // τ1 is free
        assert_eq!(free_vars.len(), 1);
    }
}
