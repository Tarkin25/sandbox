import axios from "axios";
import { getAuthToken } from "../features/auth/authSlice";

const api = axios.create({
    baseURL: process.env.NODE_ENV === "development" ? `${window.location.protocol}//${window.location.hostname}:8080` : "/api"
})

api.interceptors.request.use(request => {
    const authToken = getAuthToken();

    if(authToken) {
        request.headers = {
            ...request.headers,
            Authorization: "Bearer " + authToken
        }
    }

    return request;
})

export default api;