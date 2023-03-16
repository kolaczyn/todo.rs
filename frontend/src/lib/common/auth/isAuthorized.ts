import Cookies from 'js-cookie';

export const isAuthorized = () => !!Cookies.get('jwt');
