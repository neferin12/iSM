package ism;

import ism.Helper.Export;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;

import static ism.Helper.Import.importKurseFromCSV;
import static ism.Helper.Import.importSchuelerFromCSV;

public class App {
    private static void printVerteiler(Verteiler v) {
        v.sort();
        System.out.println(Export.getResultsAsText(v));
    }

    public static void main(String[] args) {
        if (args.length < 3) {
            System.err.println("Benutzung: jism <Wahldatei> <Seminardatei> <runs>");
        }
        ArrayList<Schueler> schuelers;
        try {
            schuelers = importSchuelerFromCSV(new File(args[0]));
        } catch (IOException e) {
            System.err.println("Schüler konnten nicht geladen werden");
            throw new RuntimeException(e);
        }
        ArrayList<Kurs>[] seminare;
        try {
            seminare = importKurseFromCSV(new File(args[1]));
        } catch (IOException e) {
            System.err.println("Seminare konnten nicht geladen werden");
            throw new RuntimeException(e);
        }
        Controller c = new Controller(schuelers, seminare, Integer.parseInt(args[2]), App::printVerteiler);
        c.run();
    }
}