import * as app_types from './app.types'
import { RegisterDeviceType } from '../../ts/auth'


export function user_port (state = 0, action: any) {
    switch (action.type) {
        case app_types.SET_APPLICATION_PORT:
            return action.payload
        default:
            return state
    }
}


export function register_auth_data(state: RegisterDeviceType | null = null, action: any){
    switch(action.type) {
        case app_types.SET_REGISTER_AUTH_DATA:
            return action.payload
        default:
            return state
    }
}