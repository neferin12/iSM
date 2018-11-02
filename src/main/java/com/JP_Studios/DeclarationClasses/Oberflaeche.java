package com.JP_Studios.DeclarationClasses;

import com.JP_Studios.Verteiler;

/**
 * Interface zum Implementieren einer Oberfläche. Sie funktioniert nur, wenn die Verteilung über den Controller erfolgt
 */
public interface Oberflaeche {
    /**
     * Methode, um den Fortschritt des Controllers an die Oberflaeche zu übermitteln
     * @param p Fortschritt in Prozent
     */
    void handleProgress(int p);

    /**
     * Methode, die ausgeführt wird, wenn der Verteilvorgang abgschlossen ist
     *
     * @param bester Der beste {@link Verteiler}
     */
    void verteilenFinished(Verteiler bester);
}
