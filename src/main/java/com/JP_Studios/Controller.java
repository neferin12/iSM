package com.JP_Studios;

import com.JP_Studios.DeclarationClasses.Oberflaeche;
import com.JP_Studios.DeclarationClasses.VerteilerComparator;

import java.util.ArrayList;
import java.util.Arrays;

/*
  Created by Julian Pollinger
 */

/**
 * Diese Klasse erstellt und verwaltet eine beliebige Anzahl an Verteildurchläufen des Algorithmus, also der Klasse {@link Verteiler}.
 */
public class Controller implements Runnable {
    private ArrayList<Schueler> schuelers;
    private ArrayList<Kurs>[] kurses;
    private int iterations;
    private Oberflaeche oberflaeche;


    private static final int PART_SIZE = 200000;

    /**
     *
     * @param schuelers Die Schueler
     * @param kurses Die Seminare
     * @param iterations Die Anzahl, wie oft der Algorithmus, also das Verteilen der Seminare durchlaufen werden soll.
     * @param oberflaeche Dies muss angegeben werden, damit der Controller Rückmeldung oder Ergebnis an die Oberfläche zuückgeben kann.
     */
    public Controller(ArrayList<Schueler> schuelers, ArrayList<Kurs>[] kurses, int iterations, Oberflaeche oberflaeche) {
        this.schuelers = schuelers;
        this.kurses = kurses;
        this.iterations = iterations;
        this.oberflaeche = oberflaeche;
    }

    /**
     * Hier werden die einzelnen Durchläufe durchgeführt. Anschließend werden die Verteildurchläufe nach MimimiPunktzahl sortiert, und der mit den wenigsten Punkten wird nach Abschluss mit {@link Oberflaeche#verteilenFinished(Verteiler)} zurück an die {@link Oberflaeche} übermittelt.
     * Während des Vorgangs wird auch der aktuelle Fortschritt mit {@link Oberflaeche#handleProgress(int)} an die {@link Oberflaeche} übermittelt. Hier wird die Anzahl nochmals in einzelne Blöcke zerlegt, die dann von der Funktion {@link Controller#calculatePart(int, int)} berechnet werden, damit der PC nicht überfordert ist.
     */
    @Override
    public void run() {
        int rest = iterations % PART_SIZE;
        int times = (iterations - rest) / PART_SIZE;

        Verteiler[] results = new Verteiler[times + 1];

        for (int i = 0; i < results.length - 1; i++) {
            results[i] = calculatePart(i * PART_SIZE, (i + 1) * PART_SIZE);
        }
        if (rest != 0) {
            results[results.length - 1] = calculatePart(iterations - (rest), iterations - 1);
        } else {
            results[results.length - 1] = new Verteiler(schuelers, kurses);
            results[results.length - 1].seminareVerteilen();
            oberflaeche.handleProgress(100);
        }


        Arrays.sort(results, new VerteilerComparator());
        oberflaeche.verteilenFinished(results[0]);
    }

    /**
     *
     * @param lowerEnd An welcher Stelle in der GesamtIterationszahl der Algorithmus beginnen soll
     * @param upperEnd An welcher Stelle in der GesamtIterationszahl der Algorithmus aufhören soll
     * @return Gibt den besten Durchlauf aus diesem Abschnitt zurück
     */

    private Verteiler calculatePart(int lowerEnd, int upperEnd) {
        Verteiler[] verteiler = new Verteiler[upperEnd - lowerEnd];
        for (int i = 0; i < verteiler.length; i++) {
            verteiler[i] = new Verteiler(schuelers, kurses);
        }
        for (int i = 0; i < verteiler.length; i++) {
            verteiler[i].seminareVerteilen();
            double iterationsD = iterations;
            double tpD = (double) (i + lowerEnd) / iterationsD;
            int p = (int) (tpD * 100);
            oberflaeche.handleProgress(p);
        }
        Arrays.sort(verteiler, new VerteilerComparator());
        return verteiler[0];
    }

}
