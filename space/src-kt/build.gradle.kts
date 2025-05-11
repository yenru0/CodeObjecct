plugins {
    val kotlinVersion = "1.9.10"
    kotlin("jvm") version kotlinVersion
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

kotlin {
    jvmToolchain(8)
}

sourceSets {
    main {
        kotlin {
            setSrcDirs(listOf("src"))
        }
    }
}

application {
    mainClass.set("MainKt")
}

tasks.named<JavaExec>("run") {
    standardInput = System.`in`
}



tasks.register<Copy>("copyJarToBuild") {
    val BUILD_DIR = "../../build"

    group = "distribution"
    description = "copy jar to top build dir"
    val jarTask = tasks.named("jar")
    from(jarTask.map { it.outputs.files })
    into(BUILD_DIR)
}

tasks.named<Jar>("jar") {
    archiveBaseName.set("kt")
    manifest {
        attributes["Main-Class"] = "MainKt"
    }
    from(configurations.runtimeClasspath.get().map { if(it.isDirectory) it else zipTree(it)})
    duplicatesStrategy = DuplicatesStrategy.EXCLUDE

    finalizedBy("copyJarToBuild")
}




