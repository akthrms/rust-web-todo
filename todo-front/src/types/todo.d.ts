export type Todo = {
  id: number;
  text: string;
  completed: boolean;
  labels: Label[];
};

export type NewTodoPayload = {
  text: string;
  labels: number[];
};

export type UpdateTodoPayload = {
  id: number;
  text: string | null;
  completed: boolean | null;
  labels: number[] | null;
};

export type Label = {
  id: number;
  name: string;
};

export type NewLabelPayload = {
  name: string;
};
