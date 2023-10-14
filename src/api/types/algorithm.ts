enum AlgorithmType {
    NaiveExponential = 'Naive Exponential'
}

// Do it this way so that we can iterate over them.
interface AlgorithmBase {
    name: AlgorithmType;
    description: string;
}

export interface NaiveExponential extends AlgorithmBase {
    name: AlgorithmType.NaiveExponential;
    description: 'Repeats questions every power of 2 days. So, 2, 4, 8, 16, 32, etc., It is naive because it does take into account only the first x good answers.';
    // The least number of days to wait before repeating a question.
    start: number;
    // The maximum number of days to wait before repeating a question.
    cap: number;
}

export type Algorithm = NaiveExponential;
export const algorithmTypes = Object.values(AlgorithmType);
