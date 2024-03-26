import React from 'react'
import LoginTwitchBtn from './LoginTwitchBtn'
import { connect } from 'react-redux'
import { RootState } from '../../store/root.reducer'
import { RegisterDeviceType } from '../../ts/auth'


interface Props {
  register_auth_data: RegisterDeviceType | null
}

const Welcome: React.FC<Props> = ({
  register_auth_data
}) => {




  return (
    <>
      {!register_auth_data ? <LoginTwitchBtn /> : null}
      {
        !register_auth_data ? null : 
        <>
          <div>
            <span> URL: </span>
            <span> {register_auth_data.verification_uri} </span>          
          </div>
          <div>
            <span> Device Code: </span>
            <span> {register_auth_data.device_code} </span>
          </div>
        </>
      }
    </>
 
  )
}

const mapStateToProps = (state: RootState) => {
  return {
    register_auth_data: state.register_auth_data
  }
}

const ConnectedWelcom = connect(mapStateToProps)(Welcome)

export default ConnectedWelcom