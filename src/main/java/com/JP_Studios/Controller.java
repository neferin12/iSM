package com.JP_Studios;

import com.JP_Studios.DeclarationClasses.Oberflaeche;
import com.JP_Studios.DeclarationClasses.VerteilerComparator;

import java.util.ArrayList;
import java.util.Arrays;

/**
 * Created by Julian Pollinger
 */

/**
 * Diese Klasse erstellt und verwaltet eine beliebige Anzahl an Verteildurchläufen des Algorithmus, also der Klasse {@link Verteiler}.
 */
public class Controller implements Runnable {
    private ArrayList<Schueler> schuelers;
    private ArrayList<Kurs>[] kurses;
    private int iterations;
    private Oberflaeche oberflaeche;
    private Verteiler verteiler[];

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
        verteiler = new Verteiler[iterations];
    }

    /**
     * Hier werden die einzelnen Durchläufe durchgeführt. Anschließend werden die Verteildurchläufe nach MimimiPunktzahl sortiert, und der mit den wenigsten Punkten wird nach Abschluss mit {@link Oberflaeche#verteilenFinished(Verteiler)} zurück an die {@link Oberflaeche} übermittelt.
     * Während des Vorgangs wird auch der aktuelle Fortschritt mit {@link Oberflaeche#handleProgress(int)} an die {@link Oberflaeche} übermittelt.
     */
    @Override
    public void run() {
        for (int i = 0; i < verteiler.length; i++) {
            verteiler[i] = new Verteiler(schuelers, kurses);
        }
        for (int i = 0; i < verteiler.length; i++) {
            verteiler[i].seminareVerteilen();
            double id= i;
            double iterationsD =iterations;
            double tpD = id/iterationsD;
            int p = (int) (tpD* 100);
            oberflaeche.handleProgress(p);
        }
        Arrays.sort(verteiler,new VerteilerComparator());
        oberflaeche.verteilenFinished(verteiler[0]);
    }

    /**
     *
     * @return Gibt eine Liste aller {@link Verteiler} zurück, die erstellt wurden.
     */
    public Verteiler[] getVerteiler() {
        return verteiler;
    }
}
