# jounce-deploy

**Deployment and environment management utilities for Jounce applications**

Provides deployment strategies (Blue-Green, Canary, Rolling), environment management, health checks, and automatic rollbacks.

## Features

- **Deployment Strategies** - Blue-Green, Canary, Rolling, Recreate
- **Environment Management** - Development, Staging, Production, Test
- **Health Checks** - Automated endpoint monitoring
- **Rollback Support** - Automatic rollback on failures
- **Deployment Tracking** - Full deployment lifecycle management

## Installation

```bash
jnc install jounce-deploy
```

## Quick Start

```jounce
use jounce_deploy::{
    Environment, DeploymentStrategy, DeploymentConfig,
    DeploymentManager, HealthCheck
};

// Create deployment manager
let manager = DeploymentManager::new(Environment::Production);

// Configure deployment
let config = DeploymentConfig::new(DeploymentStrategy::BlueGreen)
    .with_replicas(5)
    .with_rollback(true);

// Deploy
let deployment = manager.deploy("v1.0.0", config);

// Add health check
let check = HealthCheck::new("/health");
deployment = deployment.add_health_check(check);

// Start deployment
deployment = deployment.start();
```

## License

MIT
