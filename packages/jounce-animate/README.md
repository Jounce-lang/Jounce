// jounce-animate

CSS transitions, spring animations, and keyframe animations for Jounce applications.

## Features

- ✅ **CSS Transitions** - Smooth property transitions with easing
- ✅ **Easing Functions** - 22 built-in easing curves
- ✅ **Spring Animations** - Physics-based spring animations
- ✅ **Keyframe Animations** - Custom keyframe sequences
- ✅ **Animation Presets** - 9 ready-to-use animations
- ✅ **Animation Controller** - Play, pause, stop, and track progress
- ✅ **Fluent API** - Chainable builder pattern

## Installation

```bash
jnc pkg add jounce-animate
```

## Quick Start

```jounce
use jounce_animate::{Transition, EasingFunction, fade_in};

fn main() {
    // Create a CSS transition
    let transition = Transition::new("opacity", 300)
        .with_easing(EasingFunction::EaseInOut);

    println(transition.to_css());
    // "opacity 300ms ease-in-out"

    // Use a preset animation
    let animation = fade_in(500);
    println(animation.to_css_animation());
}
```

## Usage

### CSS Transitions

```jounce
use jounce_animate::{Transition, EasingFunction};

// Basic transition
let transition = Transition::new("opacity", 300);
println(transition.to_css());
// "opacity 300ms ease-in-out"

// Transition with custom easing
let transition = Transition::new("transform", 500)
    .with_easing(EasingFunction::EaseOutCubic);

// Transition with delay
let transition = Transition::new("all", 400)
    .with_easing(EasingFunction::EaseInOut)
    .with_delay(100);

println(transition.to_css());
// "all 400ms ease-in-out 100ms"
```

### Easing Functions

```jounce
use jounce_animate::EasingFunction;

// Linear
EasingFunction::Linear            // "linear"

// Standard easing
EasingFunction::EaseIn             // "ease-in"
EasingFunction::EaseOut            // "ease-out"
EasingFunction::EaseInOut          // "ease-in-out"

// Quadratic easing
EasingFunction::EaseInQuad         // cubic-bezier(0.55, 0.085, 0.68, 0.53)
EasingFunction::EaseOutQuad        // cubic-bezier(0.25, 0.46, 0.45, 0.94)
EasingFunction::EaseInOutQuad      // cubic-bezier(0.455, 0.03, 0.515, 0.955)

// Cubic easing
EasingFunction::EaseInCubic        // cubic-bezier(0.55, 0.055, 0.675, 0.19)
EasingFunction::EaseOutCubic       // cubic-bezier(0.215, 0.61, 0.355, 1)
EasingFunction::EaseInOutCubic     // cubic-bezier(0.645, 0.045, 0.355, 1)

// Quartic easing
EasingFunction::EaseInQuart        // cubic-bezier(0.895, 0.03, 0.685, 0.22)
EasingFunction::EaseOutQuart       // cubic-bezier(0.165, 0.84, 0.44, 1)
EasingFunction::EaseInOutQuart     // cubic-bezier(0.77, 0, 0.175, 1)

// Back easing (overshoots)
EasingFunction::EaseInBack         // cubic-bezier(0.6, -0.28, 0.735, 0.045)
EasingFunction::EaseOutBack        // cubic-bezier(0.175, 0.885, 0.32, 1.275)
EasingFunction::EaseInOutBack      // cubic-bezier(0.68, -0.55, 0.265, 1.55)

// Elastic easing
EasingFunction::EaseInElastic      // cubic-bezier(0.6, 0.04, 0.98, 0.335)
EasingFunction::EaseOutElastic     // cubic-bezier(0.175, 0.885, 0.32, 1.275)
EasingFunction::EaseInOutElastic   // cubic-bezier(0.68, -0.55, 0.265, 1.55)

// Bounce easing
EasingFunction::EaseInBounce       // cubic-bezier(0.6, 0.04, 0.98, 0.335)
EasingFunction::EaseOutBounce      // cubic-bezier(0.175, 0.885, 0.32, 1.275)
EasingFunction::EaseInOutBounce    // cubic-bezier(0.68, -0.55, 0.265, 1.55)
```

### Spring Animations

```jounce
use jounce_animate::{SpringConfig, SpringAnimation};

// Default spring
let config = SpringConfig::default();
let animation = SpringAnimation::new(0.0, 100.0, config);

let duration = animation.duration();  // Auto-calculated based on physics
println("Estimated duration: {}ms", duration);

// Preset spring configs
let gentle = SpringConfig::gentle();      // Gentle, smooth motion
let wobbly = SpringConfig::wobbly();      // Bouncy, playful motion
let stiff = SpringConfig::stiff();        // Fast, snappy motion
let slow = SpringConfig::slow();          // Slow, deliberate motion
let molasses = SpringConfig::molasses();  // Very slow motion

// Create animations with presets
let gentle_anim = SpringAnimation::new(0.0, 1.0, SpringConfig::gentle());
let wobbly_anim = SpringAnimation::new(0.0, 1.0, SpringConfig::wobbly());
```

### Keyframe Animations

```jounce
use jounce_animate::{Keyframe, KeyframeAnimation, EasingFunction};

// Create a custom keyframe animation
let animation = KeyframeAnimation::new("custom-fade", 1000)
    .add_keyframe(
        Keyframe::new(0.0)
            .with_property("opacity", "0")
            .with_property("transform", "translateY(20px)")
    )
    .add_keyframe(
        Keyframe::new(0.5)
            .with_property("opacity", "0.5")
            .with_property("transform", "translateY(10px)")
    )
    .add_keyframe(
        Keyframe::new(1.0)
            .with_property("opacity", "1")
            .with_property("transform", "translateY(0)")
    )
    .with_easing(EasingFunction::EaseOut)
    .with_iterations(1)
    .with_direction("normal");

// Generate CSS
let keyframes_css = animation.to_css_keyframes();
// @keyframes custom-fade { 0% { opacity: 0; transform: translateY(20px); } ... }

let animation_css = animation.to_css_animation();
// custom-fade 1000ms ease-out 0ms 1 normal
```

### Animation Presets

```jounce
use jounce_animate::{
    fade_in, fade_out, slide_in_left, slide_in_right,
    scale_in, bounce, shake, spin, pulse
};

// Fade animations
let fade_in_anim = fade_in(300);
let fade_out_anim = fade_out(300);

// Slide animations
let slide_left = slide_in_left(400);
let slide_right = slide_in_right(400);

// Scale animation
let scale = scale_in(350);

// Bounce animation
let bounce_anim = bounce(600);

// Shake animation (great for errors)
let shake_anim = shake(500);

// Spin animation (infinite)
let spin_anim = spin(1000);

// Pulse animation (infinite)
let pulse_anim = pulse(800);

// Use the animation
println(fade_in_anim.to_css_keyframes());
println(fade_in_anim.to_css_animation());
```

### Animation Controller

```jounce
use jounce_animate::{AnimationController, AnimationState};

// Create a controller
let mut controller = AnimationController::new("my-element", 1000);

// Control playback
controller.play();
controller.pause();
controller.stop();
controller.reverse();

// Check state
if controller.is_running() {
    println("Animation is running");
}

if controller.is_paused() {
    println("Animation is paused");
}

if controller.is_finished() {
    println("Animation is finished");
}

// Get progress (0.0 to 1.0)
let progress = controller.progress();
println("Progress: {:.2}%", progress * 100.0);
```

## API Reference

### Transition

```jounce
pub struct Transition {
    pub property: string,
    pub duration: int,  // milliseconds
    pub easing: EasingFunction,
    pub delay: int,     // milliseconds
}

// Methods
Transition::new(property: string, duration: int) -> Transition
transition.with_easing(easing: EasingFunction) -> Transition
transition.with_delay(delay: int) -> Transition
transition.to_css() -> string
```

### SpringConfig

```jounce
pub struct SpringConfig {
    pub stiffness: float,
    pub damping: float,
    pub mass: float,
}

// Methods
SpringConfig::default() -> SpringConfig
SpringConfig::gentle() -> SpringConfig
SpringConfig::wobbly() -> SpringConfig
SpringConfig::stiff() -> SpringConfig
SpringConfig::slow() -> SpringConfig
SpringConfig::molasses() -> SpringConfig
```

### SpringAnimation

```jounce
pub struct SpringAnimation {
    pub from: float,
    pub to: float,
    pub config: SpringConfig,
}

// Methods
SpringAnimation::new(from: float, to: float, config: SpringConfig) -> SpringAnimation
animation.duration() -> int  // Auto-calculated from spring physics
```

### Keyframe

```jounce
pub struct Keyframe {
    pub offset: float,  // 0.0 to 1.0
    pub properties: Map<string, string>,
}

// Methods
Keyframe::new(offset: float) -> Keyframe
keyframe.with_property(property: string, value: string) -> Keyframe
keyframe.to_css() -> string
```

### KeyframeAnimation

```jounce
pub struct KeyframeAnimation {
    pub name: string,
    pub keyframes: Array<Keyframe>,
    pub duration: int,
    pub easing: EasingFunction,
    pub iteration_count: int,  // 0 = infinite
    pub direction: string,
}

// Methods
KeyframeAnimation::new(name: string, duration: int) -> KeyframeAnimation
animation.add_keyframe(keyframe: Keyframe) -> KeyframeAnimation
animation.with_easing(easing: EasingFunction) -> KeyframeAnimation
animation.with_iterations(count: int) -> KeyframeAnimation
animation.with_direction(direction: string) -> KeyframeAnimation
animation.to_css_keyframes() -> string
animation.to_css_animation() -> string
```

### AnimationController

```jounce
pub struct AnimationController {
    pub element_id: string,
    pub state: AnimationState,
    pub current_time: int,
    pub duration: int,
}

// Methods
AnimationController::new(element_id: string, duration: int) -> AnimationController
controller.play()
controller.pause()
controller.stop()
controller.reverse()
controller.is_running() -> bool
controller.is_paused() -> bool
controller.is_finished() -> bool
controller.progress() -> float  // 0.0 to 1.0
```

### Preset Functions

```jounce
fade_in(duration: int) -> KeyframeAnimation
fade_out(duration: int) -> KeyframeAnimation
slide_in_left(duration: int) -> KeyframeAnimation
slide_in_right(duration: int) -> KeyframeAnimation
scale_in(duration: int) -> KeyframeAnimation
bounce(duration: int) -> KeyframeAnimation
shake(duration: int) -> KeyframeAnimation
spin(duration: int) -> KeyframeAnimation  // Infinite
pulse(duration: int) -> KeyframeAnimation // Infinite
```

## Examples

### Fade In On Load

```jounce
use jounce_animate::fade_in;

let animation = fade_in(500);

// In your HTML/CSS:
// <div class="fade-in">Content</div>
//
// <style>
// @keyframes fade-in { ... }
// .fade-in { animation: fade-in 500ms ease-in-out; }
// </style>
```

### Hover Transition

```jounce
use jounce_animate::{Transition, EasingFunction};

let transition = Transition::new("transform", 200)
    .with_easing(EasingFunction::EaseOut);

// In your CSS:
// .button {
//   transition: transform 200ms ease-out;
// }
// .button:hover {
//   transform: scale(1.1);
// }
```

### Loading Spinner

```jounce
use jounce_animate::spin;

let spinner = spin(1000);

// In your HTML/CSS:
// <div class="spinner"></div>
//
// <style>
// @keyframes spin { ... }
// .spinner { animation: spin 1000ms linear infinite; }
// </style>
```

### Shake On Error

```jounce
use jounce_animate::shake;

let shake_animation = shake(500);

// Trigger on form error:
// element.classList.add('shake');
// setTimeout(() => element.classList.remove('shake'), 500);
```

### Smooth Page Transitions

```jounce
use jounce_animate::{fade_out, fade_in};

let exit_animation = fade_out(300);
let enter_animation = fade_in(300);

// Page exit:
// - Apply fade_out to current page
// - Wait 300ms
// - Navigate to new page
// - Apply fade_in to new page
```

### Spring-Based Modal

```jounce
use jounce_animate::{SpringConfig, SpringAnimation};

let config = SpringConfig::wobbly();
let animation = SpringAnimation::new(0.0, 1.0, config);

let duration = animation.duration();

// Use spring physics for natural feel
// Modal scales from 0 to 1 with bounce
```

## Easing Curve Guide

- **Linear**: Constant speed (mechanical)
- **Ease In**: Slow start, fast end (falling)
- **Ease Out**: Fast start, slow end (natural stop)
- **Ease In Out**: Slow start and end (smooth)
- **Back**: Overshoots target (playful)
- **Elastic**: Oscillates (springy)
- **Bounce**: Bounces at end (fun)

## Spring Physics

Spring animations use real physics simulation:

```
Natural Frequency (ω) = sqrt(stiffness / mass)
Damping Ratio (ζ) = damping / (2 * sqrt(stiffness * mass))
```

- **Stiffness**: How quickly the spring returns (higher = faster)
- **Damping**: How much the spring resists motion (higher = less oscillation)
- **Mass**: Inertia of the object (higher = slower, heavier feel)

## Best Practices

1. **Choose Appropriate Durations**:
   - Micro-interactions: 100-200ms
   - Small elements: 200-400ms
   - Large elements: 400-600ms
   - Page transitions: 300-500ms

2. **Use Easing Wisely**:
   - **EaseOut** for entrances (feels responsive)
   - **EaseIn** for exits (feels natural)
   - **EaseInOut** for moves/transforms (feels smooth)

3. **Avoid Over-Animating**:
   - Don't animate everything
   - Keep animations subtle
   - Respect prefers-reduced-motion

4. **Performance**:
   - Animate `transform` and `opacity` (GPU accelerated)
   - Avoid animating `width`, `height`, `top`, `left` (causes reflow)
   - Use `will-change` sparingly

5. **Accessibility**:
   - Respect `prefers-reduced-motion`
   - Provide skip options for long animations
   - Don't rely solely on animation for feedback

## Performance Tips

```jounce
// Good: GPU-accelerated properties
Transition::new("transform", 300)
Transition::new("opacity", 300)

// Bad: Causes reflow
Transition::new("width", 300)
Transition::new("height", 300)
Transition::new("top", 300)
```

## License

MIT

## Version

0.1.0
