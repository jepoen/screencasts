mod simple_input;
use simple_input::Input;

const CRED_H_MIN: i32 = 45;
const SEM_WEEKS: i32 = 15;
const MIN_HOUR: i32 = 60;

fn main() {
    // Eingabe initialisieren
    let mut inp = Input::new();
    // Eingabe verwenden
    println!("SWS: ");
    let credit_h = inp.read_int();
    println!("Lernaufwand in %: ");
    let self_study_pcent = inp.read_int();
    // Kontrollausgabe Eingabe
    println!("Eingabe: {} SWS {} % Selbststudienaufwand",
      credit_h, self_study_pcent
    );
    // Algorithmus
    let lecture_week_min = credit_h * CRED_H_MIN;
    let self_study_week_min = lecture_week_min * self_study_pcent / 100;
    // Fehler:
    //let self_study_week_min = self_study_pcent/100 * lecture_week_min;
    let total_week_min = lecture_week_min + self_study_week_min;
    let total_sem_min = total_week_min * SEM_WEEKS;
    // Ausgabe
    println!(
        "Betreute Zeit/Woche: {} min = {}:{} Stunden",
        lecture_week_min,
        lecture_week_min / MIN_HOUR,
        lecture_week_min % MIN_HOUR
    );
    println!(
        "Selbststudienzeit/Woche: {} min = {}:{} Stunden",
        self_study_week_min,
        self_study_week_min / MIN_HOUR,
        self_study_week_min % MIN_HOUR
    );
    println!(
        "Gesamtzeit/Woche: {} min = {}:{} Stunden",
        total_week_min,
        total_week_min / MIN_HOUR,
        total_week_min % MIN_HOUR
    );
    println!(
        "Gesamtzeit/Semester: {} min = {}:{} Stunden",
        total_sem_min,
        total_sem_min / MIN_HOUR,
        total_sem_min % MIN_HOUR
    );
}
