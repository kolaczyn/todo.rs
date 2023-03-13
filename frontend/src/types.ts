export type Todo = {
	id: number;
	label: string;
	description: string | null;
	completed: boolean;
	category: TodoCategory | null;
};

export type TodoCategory = Category;

export type Category = {
	id: number;
	label: string;
	color: string;
};

export type UserDto = {
	email: string;
	id: number;
	jwt: string;
};
