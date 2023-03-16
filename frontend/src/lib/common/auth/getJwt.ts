import Cookies from 'js-cookie';

export const getJwt = () => Cookies.get('jwt');
