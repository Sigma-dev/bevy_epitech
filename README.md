# My first VampireSurvivor-like in Bevy

## 0. Before the course

[Install Rust](https://www.rust-lang.org/tools/install)

Clone this repo and run this command (the first compile is can take a while)
```
cargo run
```

Optional: Install the rust-analyzer extension for VSCode

You are good to go !

## 1. Basics

### 1.1 Cargo

Cargo is the standard build tool for Rust.

You will only need the following two commands for this project:

```bash
# Build and run the current project 
cargo run

# Check if the project could build (faster than build, use for debugging)
cargo check
```

Try `cargo run` to see if everything is working correctly

### 1.2 Entities & Components

Entities are the buildings blocks of any Bevy game, they do this by carrying components.

Components are simple structs and can't do much by themselves except hold data

### 1.3 Systems

Systems is how you will implement all game logic.

They are functions that bevy will call for you, allowing you to interact with your game world.

## 2. The real start

Right now this is just a bunch of words, but if you take a look at your [src/main.rs](./src/main.rs) file, it will make more sense.

In our app declaration, we a give our start and update functions so that bevy calls them as Systems.

Since `start()` is in the `Startup` schedule, it will be executed only once when start the application.
In it, we create an `Entity` with the `commands.spawn()` method (`commands`), and give it a `MyName` component with the name 'Pedro'.

The `MyName` component is just a simple struct that holds a string (don't .forget to derive `Component` though)

Then, our `update()` method will be called every frame because it is a system in the `Update` schedule.
In it we use a `Query` to ask for all of the existing `MyName` components.

We can then iterate over that list to print every name that exists.

This is basic foundation is how we will build everything we want in Bevy.

As your first task, I will ask that you spawn a friend for Pedro (you can choose the name), and see if you can see his name in the console.

## 3. Let's make a game

### 3.1 Visuals

First, add a `Sprite` component to Pedro so that we can see him.

You can add multiple Components when spawning an entity like this

```cpp
commands.spawn((MyName {name: "Pedro".to_string()}, ComponentA, ComponentB, etc..))
```

A fast way to make a Sprite Component is by using:
```cpp
Sprite::from_color(css::BLUE, Vec2::splat(42.))
```

You will not be able to see the sprite until you add a camera, so simply spawn another entity with the `Camera2D` component on it.

You should now be able to see something when running your game.

### 3.2 Movement

Now if we want to move Pedro, we will need a bit more logic.

First, let's create a new Player Component with a speed variable in it and add it to Pedro.

Spawning an entity with a Sprite component automatically added a Transform component to it, which allows us to move it in 3d space.

Now if we modify our query inside of our `update` system as such:
```rust
mut player_q: Query<(&mut Transform, &Player)>
```

We will ask Bevy for every entity that has both transform and player components.
The `mut` keyword you keep seeing indicates to rust that we want a mutable reference. 
Forgetting them is a classic error so make sure you have them.

We can then iterate over the entities's components like before:

```cpp
for (mut transform, player) in player_q.iter_mut () { //don't forget iter_mut otherwise we can't modify the transform.
    transform.translation.x += 1.
}
```

It moves ! (I hope)

### 3.3 Inputs

To understand inputs, we must first understand the `Resource`.

`Resource`s are similar to `Component`s, as they are also structs that can be queried and modified in systems, but the main difference is that there can be only one `Resource` of a given type at a time.

This makes them useful for tracking unique things like score, game state, etc...
Bevy also uses it to expose built-in functionalities to us like Time, Assets, etc...

The one we are most interested in right now is Inputs which can be accessed by adding a parameter to our update system:

```yaml
keys: Res<ButtonInput<KeyCode>>,
```

and then checking if our movement keys are currently pressed:

```cpp
if keys.pressed(KeyCode::KeyW) { // Bevy remaps automatically the keys, so W would be Z on an azerty keyboard
    // Move the player
}
```

Once you have implemented the movement for all four directions, you can go the next section

### 3.4 Projectiles

To test your bevy capabilities, let's try and make it so the player shoots in all four directions periodically.

We don't want Pedro to fire every frame, so one way to add a delay is to only run the system every x seconds using timers:

```cpp
  .add_system(shoot_projectiles.run_if(on_timer(Duration::from_secs_f32(0.5))))
```

If you then create a system function called shoot projectile, it should be called twice per second.

You should then be able to query for Pedro's position through the `Transform` component.
If we want to only get `Transform` from entities that also have the `Player` component, we can use the `With` query syntax:

```yaml
player_transform_q: Query<&Transform, With<Player>>
```

From there, we need to create a Projectile component, so that we can spawn four entities that we will then move in another system running every frame.

### 3.5 Enemies & Collisions

If you made it this far, first of all congratulations.

You must have by now a grasp on how to work with the ECS

Your final challenge is to add Enemies that:
- Move towards the player
- Die when hit by a projectile (I recommend using a distance based collision system for now)
- Destroy Pedro if they hit him
- Spawn periodically and randomly

To remove an entity, you can use queries to get the entity
```yaml
mut commands: Commands,
query: Query<Entity, With<Enemy>>,
```
```cpp
// Deletes every entity with the enemy component
for entity in query.iter() {
    commands.entity(entity).despawn();
}
```

## 4 What now ?

Reality can be whatever you want, you can add:

- Add health systems for the player
- Enemy loot
- Upgrades
- New weapons
- Add art to the sprites
- etc...

