export interface WTileSizeFull {
	type: string;
}

export interface WTileSize {
	type: string;
	size: number;
}

export enum KernelState {
	INITIAL,
	RENDERING,
	DONE
}

export enum OutputView {
	DEFAULT,
	FULLSCREEN
}
