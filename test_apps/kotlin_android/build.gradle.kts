import com.android.build.api.dsl.ManagedVirtualDevice
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.library") version "8.13.0"
    kotlin("android") version "2.3.21"
}

group = "dev.kreuzberg.android"
version = "0.1.0"

android {
    namespace = "dev.kreuzberg.android.e2e"
    compileSdk = 35

    defaultConfig {
        minSdk = 21
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    testOptions {
        // Gradle Managed Virtual Devices for on-device instrumented tests.
        // Run: ./gradlew pixel6api34DebugAndroidTest
        managedDevices {
            devices {
                create<ManagedVirtualDevice>("pixel6api34") {
                    device = "Pixel 6"
                    apiLevel = 34
                    systemImageSource = "aosp"
                }
            }
        }
    }
}

kotlin {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_17
    }
}

// Repositories declared in settings.gradle.kts via
// dependencyResolutionManagement (FAIL_ON_PROJECT_REPOS). Re-declaring them
// here triggers Gradle "repository was added by build file" errors.

dependencies {
    // Published Android AAR from Maven Central (verifies artifact resolution)
    implementation("dev.kreuzberg:html-to-markdown-android:3.5.1")

}
