import QtQuick
import Quickshell
import Quickshell.Io
import Quickshell.Wayland
import Quickshell.Hyprland
import QtMultimedia
import QtQuick.Shapes

PanelWindow {
    exclusionMode: ExclusionMode.Ignore
    WlrLayershell.layer: WlrLayer.Background
    anchors {
        top: true
        left: true
        right: true
        bottom: true
    }

    Process {
        id: mouseProc
        command: ["hyprctl", "cursorpos"]
        stdout: SplitParser {
            onRead: data => {
                var pos = data.split(" ");
                shaderEffect.mouse = Qt.point((parseInt(pos[0]) - 0) / 1920, parseInt(pos[1]) / 1200);
            }
        }
    }

    Timer {
        interval: 1000 / 165
        running: true
        repeat: true

        onTriggered: {
            mouseProc.running = false;
            mouseProc.running = true;
        }
    }

    ShaderEffect {
        id: shaderEffect
        width: 1920
        height: 1200
        property variant img: Image {
            id: img
            sourceSize {
                width: 1920
                height: 1200
            }
            source: "/home/ashy5000/Downloads/1920x1080-146126-dark-artwork-cityscape-futuristic-futuristic-city.jpg"
        }

        property variant mouse: Qt.point(0, 0)

        vertexShader: "shader.vert.qsb"
        fragmentShader: "shader.frag.qsb"
    }
}
