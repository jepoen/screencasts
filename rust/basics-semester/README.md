# Problem: Lernaufwand Studienfach

Eingabe:
- Betreute Zeit (SWS, credit hours): cred_h
- Zeitaufwand Selbststudium Prozent: self_study_pcent

Ausgabe:
- Selbststudienzeit/Woche in min: self_study_week_min
- Betreute Zeit/Woche in min: lecture_week_min
- Gesamtzeit/Woche in min: total_week_min
- Gesamtzeit/Semester in min: total_sem_min
- Umrechnung in Stunden:Minuten

Konstanten:
- Dauer SWS in min: CRED_H_MIN
- Dauer Semester in Wochen: SEM_WEEKS
- Minuten pro Stunde: MIN_HOUR

Algorithmus:

lecture_week_min    = cred_h * CRED_H_MIN
self_study_week_min = cred_h * CRED_H_MIN * self_study_pcent/100
                    = lecture_week_min * self_study_pcent/100
total_week_min      = lecture_week_min + self_study_week_min
total_sem_min       = total_week_min * SEM_WEEKS

min -> Stunden:Minuten

hours   = min/MIN_HOUR (ganzzahlige Division)
min_rem = min - hours * MIN_HOUR

Beispiel: AuP (cred_h = 6, stud_study_pcent = 80)

self_study_week_min = 6 * 45 * 80 / 100 = 270 * 80 / 100 = 216

hours   = 216 / 60 = 3
min_rem = 216 - 3 * 60 = 36

