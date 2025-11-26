Create android libs using the command in justfile

```sh
just create-android-libs
```

Then open platforms/android/android-app with Android Studio.
Run the app with a physical device to see if UI flickers.

You can also remove the PostProcessSettings component from the camera to check whether UI flickers without the post processing effect. 
```rust
    // src/camera.rs

    commands.spawn((MainCamera, Camera2d, PostProcessSettings { time: 0.0 }));
}
```
