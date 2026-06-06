import java.net.HttpURLConnection
import java.net.URL
import java.util.zip.ZipFile
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.library") version "8.13.0"
    kotlin("android") version "2.2.0"
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
        // Host JVM unit tests: no Android device/emulator required.
        // Tests run against the published AAR and JVM-side deps via `gradle test`.
        unitTests {
            isReturnDefaultValues = true
        }
    }
}

kotlin {
    // Set JVM target for compilation. gradle.properties enables auto-detection
    // of host JDK installations so Gradle uses the available JDK version on the
    // build machine, preventing provisioning failures when the target version is not installed.
    compilerOptions {
        jvmTarget = JvmTarget.JVM_17
    }
}

// Repositories declared in settings.gradle.kts via
// dependencyResolutionManagement (FAIL_ON_PROJECT_REPOS). Re-declaring them
// here triggers Gradle "repository was added by build file" errors.

dependencies {
    // Published Android AAR from Maven Central (verifies artifact resolution)
    implementation("dev.kreuzberg:html-to-markdown-android:3.6.0-rc.16")
    // Jackson for JSON assertion helpers
    testImplementation("com.fasterxml.jackson.core:jackson-annotations:2.18.2")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.18.2")
    testImplementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.18.2")

    // jackson-module-kotlin registers constructors/properties for Kotlin data
    // classes, which have no default constructor and cannot be deserialized by
    // plain Jackson without this module.
    testImplementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.18.2")

    // jspecify for null-safety annotations on wrapped types
    testImplementation("org.jspecify:jspecify:1.0.0")

    // Kotlin coroutines for async test helpers
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")

    // JUnit 5 API and engine
    testImplementation("org.junit.jupiter:junit-jupiter-api:6.1.0")
    testImplementation("org.junit.jupiter:junit-jupiter-engine:6.1.0")


    // Kotlin stdlib test helpers
    testImplementation(kotlin("test"))

    // JNA for loading the native library from java.library.path
    testImplementation("net.java.dev.jna:jna:5.18.1")

}

tasks.register("verifyAarPublished") {
    description = "Verify the published Android AAR contains jniLibs and classes.jar"
    doLast {
        val aarCoord = "dev.kreuzberg:html-to-markdown-android:3.6.0-rc.16"
        val (groupId, artifactId, version) = run {
            val parts = aarCoord.split(':')
            Triple(parts[0], parts[1], parts[2])
        }
        val aarFileName = "${artifactId}-${version}.aar"
        val mavenUrl = "https://repo1.maven.org/maven2/${groupId.replace('.', '/')}/${artifactId}/${version}/${aarFileName}"
        val aarFile = layout.buildDirectory.file("tmp/${aarFileName}").get().asFile

        println("Downloading AAR from Maven Central: ${mavenUrl}")
        aarFile.parentFile.mkdirs()

        val connection = URL(mavenUrl).openConnection() as HttpURLConnection
        connection.requestMethod = "GET"
        connection.connect()

        if (connection.responseCode != 200) {
            throw GradleException("Failed to download AAR: HTTP ${connection.responseCode}")
        }

        connection.inputStream.use { input ->
            aarFile.outputStream().use { output ->
                input.copyTo(output)
            }
        }

        println("Verifying AAR contents...")
        ZipFile(aarFile).use { zip ->
            val entries = zip.entries().toList()
            val hasJniLibs = entries.any { it.name.startsWith("jniLibs/") }
            val hasClasses = entries.any { it.name == "classes.jar" }

            if (!hasJniLibs) {
                throw GradleException("AAR missing jniLibs directory")
            }
            if (!hasClasses) {
                throw GradleException("AAR missing classes.jar")
            }

            val abiDirs = entries
                .filter { it.name.startsWith("jniLibs/") }
                .map { it.name.substringAfter("jniLibs/").substringBefore("/") }
                .filter { it.isNotEmpty() }
                .distinct()

            println("  + jniLibs: YES")
            println("  + classes.jar: YES")
            println("  + Android ABIs: " + abiDirs.sorted().joinToString(", "))
            println("\nAAR verification PASSED!")
        }
    }
}

tasks.withType<Test> {
    useJUnitPlatform()
    dependsOn("verifyAarPublished")
}
