import React from 'react'
import { useNavigate } from 'react-router-dom'
import { connect } from 'react-redux'
import * as appActions from '../../store/app/app.actions'

type Props = {
    request_device_authorization: () => Promise<any>
}

const LoginTwitchBtn:React.FC<Props> = ({
    request_device_authorization
    
}) => {

    let nav = useNavigate();

    const handle_login = async () => {
     await request_device_authorization()
    }
    
        
  return (
    <button onClick = {handle_login}> Login with Twitch </button>
  )
}

const ConnectedLoginTwitchBtn = connect(null, ({
  request_device_authorization: appActions.request_device_authorization
}))(LoginTwitchBtn)


export default ConnectedLoginTwitchBtn