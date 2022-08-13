
function filterComponentInfos(infos: ComponentInfo[], prefix: string): string[] {
    return infos
        .filter(info => info.name.startsWith(prefix))
        .map(info => info.name.replace(prefix, ""));
}


let firstIteration = true;
let i = 0;
function run() {
    if (firstIteration) {
        firstIteration = false;

        // info("Components: " + filterComponentInfos(world.components, "bevy_transform::"));
        // info("Resources: " + filterComponentInfos(world.resources, "breakout::").join(", "));
    }


    i++;
    if (i % 60 == 0) {
        let timeId = world.resources.find(info => info.name == "bevy_time::time::Time").id;
        let time = world.resource(timeId);
        info(time.delta_seconds.toString());
    }
}
