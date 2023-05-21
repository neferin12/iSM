package ism;

import ism.DeclarationClasses.Comparator.VerteilerComparator;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.function.Consumer;


/**
 * Diese Klasse erstellt und verwaltet eine beliebige Anzahl an Verteildurchläufen des Algorithmus, also der Klasse {@link Verteiler}.
 */
public class Controller implements Runnable {
    private static int PART_SIZE = 200000;
    private ArrayList<Schueler> schuelers;
    private ArrayList<Kurs>[] kurses;
    private int iterations;
    private Consumer<Verteiler> callback;

    /**
     * @param schuelers   Die Schueler
     * @param kurses      Die Seminare
     * @param iterations  Die Anzahl, wie oft der Algorithmus, also das Verteilen der Seminare durchlaufen werden soll.
     */
    public Controller(ArrayList<Schueler> schuelers, ArrayList<Kurs>[] kurses, int iterations, Consumer<Verteiler> callback) {
        this.schuelers = schuelers;
        this.kurses = kurses;
        this.iterations = iterations;
        this.callback = callback;
    }

    public static int getPartSize() {
        return PART_SIZE;
    }

    public static void setPartSize(int partSize) {
        PART_SIZE = partSize;
    }

    /**
     * Hier werden die einzelnen Durchläufe durchgeführt. Anschließend werden die Verteildurchläufe nach MimimiPunktzahl sortiert.
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
        }


        Arrays.sort(results, new VerteilerComparator());

        this.callback.accept(results[0]);
    }

    /**
     * @param lowerEnd An welcher Stelle in der GesamtIterationszahl der Algorithmus beginnen soll
     * @param upperEnd An welcher Stelle in der GesamtIterationszahl der Algorithmus aufhören soll
     * @return Gibt den besten Durchlauf aus diesem Abschnitt zurück
     */

    private Verteiler calculatePart(int lowerEnd, int upperEnd) {
        Verteiler bester = null;
        for (int i = 0; i < upperEnd-lowerEnd; i++) {
             Verteiler v = new Verteiler(schuelers, kurses);
             v.seminareVerteilen();
            if (bester == null || bester.punktzahl > v.punktzahl) {
                bester = v;
            }
        }
        return bester;
    }


}
