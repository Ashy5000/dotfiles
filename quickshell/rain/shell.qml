import Quickshell
import Quickshell.Io
import QtQuick

// PanelWindow {
//     color: "transparent"
//     // surfaceFormat: false
//   anchors {
//     // top: true
//     left: true
//     bottom: true
//     right: true
// }

// margins {
//     top: 0
//     bottom: 0
//     left: 0
//     right: 0
// }

// implicitHeight: 1000
// exclusionMode: ExclusionMode.Ignore



//   aboveWindows: false

//   mask: Region { item: maskRect }

//   Rectangle {
//       id: maskRect
//       anchors.centerIn: parent
//       width: 100000
//       height: 100000
//       color: "transparent"
//   }

//   Text {
//     id: clock
//     anchors.centerIn: parent
//     color: "blue"

//     FileView {
//         id: matrixFile
//         path: Qt.resolvedUrl("/home/ashy5000/etc/i3_matrix.txt")
//         watchChanges: true
//         blockLoading: true
//     }

//     // use a timer to rerun the process at an interval
//     Timer {
//       // 1000 milliseconds is 1 second
//       interval: 100

//       // start the timer immediately
//       running: true

//       // run the timer again when it ends
//       repeat: true


//       // when the timer is triggered, set the running property of the
//       // process to true, which reruns it if stopped.
//       onTriggered: {
//           matrixFile.reload();
//           clock.text = matrixFile.text();
//       }
//     }
//   }
// }

ShellRoot {
	LockContext {
		id: lockContext
		onUnlocked: Qt.quit();
	}

	FloatingWindow {
		LockSurface {
			anchors.fill: parent
			context: lockContext
		}
	}

	// exit the example if the window closes
	Connections {
		target: Quickshell

		function onLastWindowClosed() {
			Qt.quit();
		}
	}
}

