export type Todo = {
	id: number;
	label: string;
	description: string | null;
	completed: boolean;
	category_id: number;
};

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
