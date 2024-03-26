import { combineReducers } from "redux";

import { user_port, register_auth_data } from './app/app.reducer'



const rootReducer = combineReducers({
    user_port,
    register_auth_data
});

export type RootState = ReturnType<typeof rootReducer>;

export default rootReducer;

