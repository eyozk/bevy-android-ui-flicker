create-android-libs:
	cd platforms/android && \
	cargo ndk -t armeabi-v7a -t arm64-v8a -o ./android-app/app/src/main/jniLibs build
