# Tools für Screencasts

## subtitle

- Video-Untertitel mit Whisper
- Abgeleitet von [DigitalOcean](https://www.digitalocean.com/community/tutorials/how-to-generate-and-add-subtitles-to-videos-using-python-openai-whisper-and-ffmpeg)
- Installation: [faster-whisper](https://github.com/SYSTRAN/faster-whisper)
  in Python-Venv
- Hinweis zu CUDA 11 beachten
- Python-Venv benötigt noch `ffmpeg-python`

Für libcuda/libcublas 11:
~~~
# Virtuelle Umgebung erstellen
python -m venv {whisper-venv}
# und aktivieren
. {whisper-venv}/bin/activate
# notwendige Pakete installieren (getestet unter Linux-Mint)
pip install -r requirements.txt
# ctranslate für libcudnn11 überschreiben
pip install --force-reinstall ctranslate2==3.24.0
~~~

- Python-Script `subtitle.py` in die Venv kopieren
- Pfad für venv und Python-Script in Shell-Script `subtitle` anpassen
- Shell-Script in Pfad aufnehmen
- Shell-Script aktiviert virtuelle Umgebung und startet dann das Python-Script,
  das in diese kopiert wird

Verwendung (`subtitle --help`):

~~~
usage: subtitle [-h] [-m MODEL] [-pa] [-f FORMAT] filename

Generate video suptitles using whisper

positional arguments:
  filename              video file name

options:
  -h, --help            show this help message and exit
  -m MODEL, --model MODEL
                        whisper model name
  -pa, --preserve-audio
                        preserve audio file
  -f FORMAT, --format FORMAT
                        Output format (vtt or srt)
~~~
