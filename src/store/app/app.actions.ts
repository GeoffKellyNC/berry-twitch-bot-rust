import * as app_types from './app.types'
import { invoke } from '@tauri-apps/api'
import { Dispatch } from 'redux';
import { RegisterDeviceType } from '../../ts/auth'

const check_application_port = async (): Promise<string> => {

    let port = await invoke("check_port");

    return "Hello"
}

export const request_device_authorization = () => async (dispatch: Dispatch): Promise<RegisterDeviceType> => {

    console.log("Requesting device authorization");
    
    let device: RegisterDeviceType = await invoke("request_device_authorization");

    console.log("Device: ", device);

    dispatch({
        type: app_types.SET_REGISTER_AUTH_DATA,
        payload: device
    })

    return device
        
}

