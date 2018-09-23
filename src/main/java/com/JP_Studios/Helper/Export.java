package com.JP_Studios.Helper;

import com.JP_Studios.DeclarationClasses.GlobalConstants;
import com.JP_Studios.Kurs;
import com.JP_Studios.Schueler;
import com.JP_Studios.Verteiler;

import java.util.ArrayList;

public class Export {
    /**
     * @return Ausgabe der Ergebnisse als Text
     */
    public static String getResultsAsText(Verteiler verteiler) {
        ArrayList<Schueler>[] �berschuss = verteiler.get�berschuss();
        String results = "";
        results += "______________Seminare______________ " + System.lineSeparator()+"W-Seminare:"+ System.lineSeparator();
        ArrayList<Kurs> WSeminar = verteiler.getKurse(GlobalConstants.W_SEMINAR);
        for (Kurs kurs : WSeminar) {
            results+="  "+ kurs.getName()+":"+ System.lineSeparator();
            ArrayList<Schueler> schuelers = kurs.getSchueler();
            for (Schueler schueler : schuelers) {
                results += "        "+ schueler.name+ System.lineSeparator();
            }
        }
        results += System.lineSeparator()+"P-Seminare:"+ System.lineSeparator();

        ArrayList<Kurs> PSeminar = verteiler.getKurse(GlobalConstants.P_SEMINAR);
        for (Kurs kurs : PSeminar) {
            results+="  "+ kurs.getName()+":"+ System.lineSeparator();
            ArrayList<Schueler> schuelers = kurs.getSchueler();
            for (Schueler schueler : schuelers) {
                results += "        "+ schueler.name+ System.lineSeparator();
            }
        }

        results += System.lineSeparator() + System.lineSeparator() + "______________Sch�ler_ohne_Seminar______________ "+ System.lineSeparator()+"W-Seminare:"+ System.lineSeparator();
        for (Schueler schueler : �berschuss[GlobalConstants.W_SEMINAR]) {
            results += "        "+ schueler.name+ System.lineSeparator();
        }
        results += "P-Seminare:" + System.lineSeparator();
        for (Schueler schueler : �berschuss[GlobalConstants.P_SEMINAR]) {
            results += "        "+ schueler.name+ System.lineSeparator();
        }

        results += System.lineSeparator() + System.lineSeparator() + System.lineSeparator() + "Mimimiquote: " + verteiler.punktzahl;

        return results;
    }
}
