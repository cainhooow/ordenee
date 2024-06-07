import * as tf from '@tensorflow/tfjs';

type Vocab = Record<string, number>;

export class OrdeneeNer {
	private labels = [' ', 'PERSON', 'PAYMENT', 'CPF', 'ADDRESS'];

	public async result(input: string) {
		const { inputTensor, model, tokens } = await this._configure(input);

		const prediction = model.predict(inputTensor);
		/** @ts-ignore */
		const predictedLabels = [...prediction.argMax(-1).dataSync()].map(
			(idx) => this._idxToLabels()[idx]
		);

		return tokens.map((token, idx) => ({
			token,
			label: predictedLabels[idx] ? Object.values(predictedLabels[idx])[0] : ' '
		}));
	}

	private async _configure(input: string) {
		const vocab = await this._loadVocab();
		const model = await this._loadmodel();

		const tokens = this._tokenizer(input);
		const indices = this._sentencesToIdx([tokens], vocab);
		const padded = this._padSequences(indices);
		const inputTensor = tf.tensor2d(padded, [1, 50]);

		return { inputTensor, model, tokens };
	}

	private _loadVocab() {
		return fetch('/models/ordenee-model/vocab.json')
        .then(async (res) => {
            const vocab = await res.json();
            return vocab as Vocab;
        })
	}
	private async _loadmodel() {
		try {
			return await tf.loadLayersModel('/models/ordenee-model/model.json');
		} catch (err) {
			throw err;
		}
	}
	private _padSequences(
		sequences: string[][] | number[][],
		maxLen: number = 50,
		padding: number = 0
	): number[][] {
		return sequences.map((seq) => {
			const padSeq = Array(maxLen).fill(padding);
			for (let i = 0; i < Math.min(seq.length, maxLen); i++) {
				padSeq[i] = seq[i];
			}
			return padSeq;
		});
	}

	private _tokenizer(sentence: string) {
		return sentence.split(' ');
	}

	private _sentencesToIdx(sentences: string[][], vocab: Vocab) {
		return sentences.map((sentence) => {
			return sentence.map((word) => vocab[word] || 0);
		});
	}

	private _idxToLabels() {
		return this.labels.map((label, idx) => ({ [idx]: label }));
	}
}
