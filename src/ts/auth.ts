

export interface RegisterDeviceType {
    verification_uri: string,
    user_code: string,
    device_code: string,
    expires_in: number,
    interval: number,
    has_verified: boolean
}