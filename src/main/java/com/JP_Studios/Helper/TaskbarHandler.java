package com.JP_Studios.Helper;

import java.awt.*;

/**
 * Created by Julian Pollinger
 */
public class TaskbarHandler {
    private static Taskbar taskbar = Taskbar.getTaskbar();
    public static void setTaskbarProgress(int progress, Frame frame) {
        if (Taskbar.isTaskbarSupported()) {
            taskbar.setWindowProgressValue(frame, progress);
        }
    }

    public static void setTaskbarFinished(Frame frame) {
        if (Taskbar.isTaskbarSupported()) {
            taskbar.requestWindowUserAttention(frame);
            taskbar.setWindowProgressState(frame, Taskbar.State.OFF);
        }
    }
}
