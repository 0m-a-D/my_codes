#include <stdio.h>
#include <string.h>
#include <espeak-ng/speak_lib.h>

void textToSpeech(const char* text) {
    espeak_Initialize(AUDIO_OUTPUT_PLAYBACK, 0, NULL, 0);
    unsigned int flags = espeakCHARS_AUTO | espeakENDPAUSE;

    espeak_Synth(text, strlen(text) + 1, 0, POS_CHARACTER, 0, flags, NULL, NULL);

    espeak_Synchronize();

    espeak_Terminate();
}

int main() {
    const char* text = "please don't talk so much morning morning!";

    textToSpeech(text);

    return 0;
}
