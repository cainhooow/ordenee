import { Tokenizer, loadQnA } from '@tensorflow-models/universal-sentence-encoder';
import * as tf from '@tensorflow/tfjs';

export default class OrdeneeSentencer {
	private queries: string[] = [];
	private responses: string[] = [];

	public query(input: string, responses: string[]) {
		const response = tf.ready().then(async () => {
			return await this._prepare(input, responses);
		});

		return response;
	}

	private async _prepare(input: string, responses: string[]) {
		this.queries.push(input);
		this.responses = responses;

		const model = await loadQnA();

		const inputModel = {
			queries: this.queries,
			responses: this.responses
		};

		const embeddings = model.embed(inputModel);

		const queryEmbedding = embeddings.queryEmbedding.arraySync() as number[][];
		const responseEmbedding = embeddings.responseEmbedding.arraySync() as number[][];
		const scoresTensor = tf.matMul(tf.tensor(queryEmbedding), tf.tensor(responseEmbedding), false, true);
		const scores = scoresTensor.arraySync() as number[][];

		const predictRes: string = this._predict(scores[0], inputModel, 1);

		return predictRes;
	}

	private _predict(scores: any, input: any, maxInput?: number) {
		if (typeof maxInput !== 'undefined' && maxInput === 1) {
			const maxIndex = scores.indexOf(Math.max(...scores));
			return input.responses[maxIndex];
		}

		return scores.map((queryScores: any) => {
			const maxIndex = queryScores.indexOf(Math.max(...queryScores));
			return input.responses[maxIndex];
		});
	}
}
