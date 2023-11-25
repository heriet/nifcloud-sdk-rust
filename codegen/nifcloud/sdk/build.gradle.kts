plugins {
    id("software.amazon.smithy").version("0.6.0")
}

val smithyVersion: String by project

val outputDir = buildDir.resolve("nifcloud-sdk")
val sdkOutputDir = outputDir.resolve("sdk")
var services = arrayOf("storage")

dependencies {
    implementation("software.amazon.smithy.rust.codegen:codegen-client:0.1.0")

    implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
    implementation("software.amazon.smithy:smithy-model:$smithyVersion")
    implementation("software.amazon.smithy:smithy-validation-model:$smithyVersion")
}

tasks.register<Delete>("deleteSdk") {
    delete(
        fileTree(outputDir) {
            include("**/*.*")
        },
    )
}

tasks.register("relocateServices") {
    description = "relocate NIFCLOUD Services SDK"
    dependsOn("smithyBuildJar")

    doLast {
        services.forEach {
            logger.info("Relocating ${it}...")
            copy {
                from("$buildDir/smithyprojections/sdk/${it}/rust-client-codegen")
                into(sdkOutputDir.resolve(it))
            }
        }
    }
    inputs.dir(layout.buildDirectory.dir("smithyprojections/sdk/"))
    outputs.dir(sdkOutputDir)
}

tasks["assemble"].apply {
    dependsOn(
        "deleteSdk",
        "smithyBuildJar",
        "relocateServices",
    )
    outputs.upToDateWhen { false }
}
