import type { Readable, Writable } from 'svelte/store';

type DistributeWritable<T> = {
  [P in keyof T]: Writable<T[P]>;
};

type DistributeReadable<T> = {
  [P in keyof T]: Readable<T[P]>;
};

interface ReadOnlyAudioState {
  playing: boolean;
  ended: boolean;
  duration: number;
}

interface AudioState {
  currentTime: number;
  volume: number;
  muted: boolean;
  repeat: boolean;
  playbackRate: number;
  paused: boolean;
}

export type WritableAudioState = DistributeWritable<AudioState>;
export type ReadableAudioState = DistributeReadable<ReadOnlyAudioState>;

export type AudioContext = WritableAudioState &
  ReadableAudioState & {
    seekBy: (n: number) => void;
    seekTo: (t: number) => void;
  };
