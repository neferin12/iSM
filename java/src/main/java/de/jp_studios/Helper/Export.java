package de.jp_studios.Helper;

import de.jp_studios.DeclarationClasses.GlobalConstants;
import de.jp_studios.Kurs;
import de.jp_studios.Schueler;
import de.jp_studios.Verteiler;

import java.io.File;
import java.io.IOException;
import java.io.PrintStream;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;

/**
 * Klasse zum exportieren der Ergebnisse
 */
public abstract class Export {
    /**
     * @param verteiler Der {@link Verteiler}, dessen Ergebnisse ausgegeben werden sollen
     * @return Ausgabe der Ergebnisse als Text
     */
    @Deprecated
    public static String getResultsAsText(Verteiler verteiler) {
        ArrayList<Schueler>[] ueberschuss = verteiler.getUeberschuss();
        String results = "";
        results += "______________Seminare______________ " + System.lineSeparator() + "W-Seminare:" + System.lineSeparator();
        ArrayList<Kurs> WSeminar = verteiler.getKurse(GlobalConstants.W_SEMINAR);
        for (Kurs kurs : WSeminar) {
            results += "  " + kurs.getName() + ":" + System.lineSeparator();
            ArrayList<Schueler> schuelers = kurs.getSchueler();
            for (Schueler schueler : schuelers) {
                results += "        " + schueler.name + System.lineSeparator();
            }
        }
        results += System.lineSeparator() + "P-Seminare:" + System.lineSeparator();

        ArrayList<Kurs> PSeminar = verteiler.getKurse(GlobalConstants.P_SEMINAR);
        for (Kurs kurs : PSeminar) {
            results += "  " + kurs.getName() + ":" + System.lineSeparator();
            ArrayList<Schueler> schuelers = kurs.getSchueler();
            for (Schueler schueler : schuelers) {
                results += "        " + schueler.name + System.lineSeparator();
            }
        }

        results += System.lineSeparator() + System.lineSeparator() + "______________Schüler_ohne_Seminar______________ " + System.lineSeparator() + "W-Seminare:" + System.lineSeparator();
        for (Schueler schueler : ueberschuss[GlobalConstants.W_SEMINAR]) {
            results += "        " + schueler.name + System.lineSeparator();
        }
        results += "P-Seminare:" + System.lineSeparator();
        for (Schueler schueler : ueberschuss[GlobalConstants.P_SEMINAR]) {
            results += "        " + schueler.name + System.lineSeparator();
        }

        results += System.lineSeparator() + System.lineSeparator() + System.lineSeparator() + "Mimimiquote: " + verteiler.punktzahl;

        return results;
    }

    /**
     * @param verteiler Der {@link Verteiler}, dessen Ergebnisse ausgegeben werden sollen
     * @param path      Der Pfad, zu dem die Dateien ausgegeben werden sollen
     * @throws IOException Falls das Exportieren nicht möglich war
     */
    public static void saveCSVFiles(Verteiler verteiler, String path) throws IOException {
        ArrayList<Schueler> schuelers = verteiler.getSchueler();
        File folder = new File(path + "/iSM Export");
        if (!folder.exists()) {
            final boolean success = folder.mkdirs();
            if (!success) {
                throw new IOException("Erstellen des Ordners fehlgeschlagen");
            }
        }
        String outputSchueler = "Name;W-Seminar;P-Seminar" + System.lineSeparator();
        for (Schueler schueler : schuelers) {
            String wSeminar = "KEINES";
            String pSeminar = "KEINES";

            if (schueler.kurse()[GlobalConstants.W_SEMINAR] != -1) {
                wSeminar = verteiler.getKurse(GlobalConstants.W_SEMINAR).get(schueler.kurse()[GlobalConstants.W_SEMINAR]).getName();
            } else {
                int[] wünsche = schueler.getWseminarwahl();
                wSeminar += " (Wünsche: ";
                for (int i : wünsche) {
                    wSeminar += " " + i + " ";
                }
                wSeminar += ")";
            }

            if (schueler.kurse()[GlobalConstants.P_SEMINAR] != -1) {
                pSeminar = verteiler.getKurse(GlobalConstants.P_SEMINAR).get(schueler.kurse()[GlobalConstants.P_SEMINAR]).getName();
            } else {
                int[] wünsche = schueler.getPseminarwahl();
                pSeminar += " (Wünsche: ";
                for (int i : wünsche) {
                    pSeminar += " " + i + " ";
                }
                pSeminar += ")";
            }
            outputSchueler += schueler.name + ";" + wSeminar + ";" + pSeminar + System.lineSeparator();
        }

        File output = new File(path + "/iSM Export/Schüler.csv");
        output.createNewFile();
        try (PrintWriter out = new PrintWriter(new PrintStream(output, StandardCharsets.ISO_8859_1))) {
            out.print(outputSchueler);
            out.flush();
        }


        folder = new File(path + "/iSM Export/Seminare/P-Seminare");
        if (!folder.exists()) {
            final boolean success = folder.mkdirs();
            if (!success) {
                throw new IOException("Erstellen des Ordners fehlgeschlagen");
            }
        }
        folder = new File(path + "/iSM Export/Seminare/W-Seminare");
        if (!folder.exists()) {
            final boolean success = folder.mkdirs();
            if (!success) {
                throw new IOException("Erstellen des Ordners fehlgeschlagen");
            }
        }

        ArrayList<Kurs>[] kurse = verteiler.getKurse();
        ArrayList<Kurs> pSeminare = kurse[GlobalConstants.P_SEMINAR];
        ArrayList<Kurs> wSeminare = kurse[GlobalConstants.W_SEMINAR];

        for (Kurs kurs : wSeminare) {
            String textOutput = "";
            ArrayList<Schueler> schüler = kurs.getSchueler();
            for (Schueler schueler : schüler) {
                textOutput += schueler.name + System.lineSeparator();
            }
            output = new File(path + "/iSM Export/Seminare/W-Seminare/" + kurs.getName() + ".csv");
            output.createNewFile();
            try (PrintWriter out = new PrintWriter(new PrintStream(output, StandardCharsets.ISO_8859_1))) {
                out.print(textOutput);
                out.flush();
            }
        }

        for (Kurs kurs : pSeminare) {
            String textOutput = "";
            ArrayList<Schueler> schüler = kurs.getSchueler();
            for (Schueler schueler : schüler) {
                textOutput += schueler.name + System.lineSeparator();
            }
            output = new File(path + "/iSM Export/Seminare/P-Seminare/" + kurs.getName() + ".csv");
            output.createNewFile();
            try (PrintWriter out = new PrintWriter(new PrintStream(output, StandardCharsets.ISO_8859_1))) {
                out.print(textOutput);
                out.flush();
            }
        }

    }
}
