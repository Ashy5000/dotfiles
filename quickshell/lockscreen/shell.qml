import Quickshell

ShellRoot {
	// This stores all the information shared between the lock surfaces on each screen.
	LockContext {
		id: lockContext

		onUnlocked: {
			// Unlock the screen before exiting, or the compositor will display a
			// fallback lock you can't interact with.
			lock.locked = false;

			Qt.quit();
		}
	}
}
