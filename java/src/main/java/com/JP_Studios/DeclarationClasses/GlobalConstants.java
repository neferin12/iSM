package com.JP_Studios.DeclarationClasses;

/**
 * Alle wichtigen Konstanten
 */
public abstract class GlobalConstants {

    public static final int W_SEMINAR = 0;
    public static final int P_SEMINAR = 1;

    public static int ERSTE_WAHL = 0;
    public static int ZWEITE_WAHL = 5;
    public static int DRITTE_WAHL = 10;
    public static int KEINE_WAHL = 20;

    public static void setErsteWahl(int miMiMiPunktzahl) {
        ERSTE_WAHL = miMiMiPunktzahl;
    }

    public static void setZweiteWahl(int miMiMiPunktzahl) {
        ZWEITE_WAHL = miMiMiPunktzahl;
    }

    public static void setDritteWahl(int miMiMiPunktzahl) {
        DRITTE_WAHL = miMiMiPunktzahl;
    }

    public static void setKeineWahl(int miMiMiPunktzahl) {
        KEINE_WAHL = miMiMiPunktzahl;
    }
}
