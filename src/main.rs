use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    pr_info,
    msg::common_interfaces::{geometry_msgs, std_msgs},
};
use async_std;
use dualshock_driver::{DualShock4, DualShock4Driver, BLE, SERIAL};
use ros2_rust_util::get_str_parameter;

#[async_std::main]
async fn main()->Result<(), DynError>
{
    let ctx = Context::new()?;
    let node = ctx.create_node("PS4ROS2", None, Default::default())?;
    let log = Logger::new(node.get_name().unwrap().as_str());

    let twist_publisher = node.create_publisher::<geometry_msgs::msg::Twist>("/ps4/twist", None)?;
    let pub_01 = node.create_publisher::<std_msgs::msg::Float32>("/ps4/one", None)?;
    let pub_02 = node.create_publisher::<std_msgs::msg::Float32>("/ps4/two", None)?;
    

    let mode_name = get_str_parameter(node.get_name().unwrap().as_str(), "mode", "ble");
    let p_01 = get_str_parameter(node.get_name().unwrap().as_str(), "01_plus", "dpad.up");
    let m_01 = get_str_parameter(node.get_name().unwrap().as_str(), "01_minus", "dpad.down");

    let p_02 = get_str_parameter(node.get_name().unwrap().as_str(), "02_plus", "btn.r1");
    let m_02 = get_str_parameter(node.get_name().unwrap().as_str(), "02_minus", "btn.l1");

    let mode = name_to_mode(&mode_name);
    let mut driver = DualShock4Driver::new(mode).unwrap();

    pr_info!(log, "Start {}", node.get_name().unwrap().as_str());
    loop {
        let con = driver.read().await.unwrap();

        let mut msg = geometry_msgs::msg::Twist::new().unwrap();
        let mut msg_01 = std_msgs::msg::Float32::new().unwrap();
        let mut msg_02 = std_msgs::msg::Float32::new().unwrap();

        msg.linear.x = con.sticks.left_x as f64;
        msg.linear.y = con.sticks.left_y as f64;
        msg.angular.z = con.sticks.right_x as f64;

        if name_to_bool(p_01.as_str(), &con)
        {
            msg_01.data = 1.0;
        }
        else if name_to_bool(m_01.as_str(), &con)
        {
            msg_01.data = -1.0;
        }

        if name_to_bool(p_02.as_str(), &con)
        {
            msg_02.data = 1.0;
        }
        else if name_to_bool(m_02.as_str(), &con)
        {
            msg_02.data = -1.0;
        }

        let _ = twist_publisher.send(&msg).unwrap();
        let _ = pub_01.send(&msg_01).unwrap();
        let _ = pub_02.send(&msg_02).unwrap();
    }
}

fn name_to_mode(name:&str)->u8
{
    if name == "ble"
    {
        BLE
    }
    else if name == "serial"
    {
        SERIAL
    }
    else {
        0
    }
}

fn name_to_bool(name:&str, con:&DualShock4)->bool
{
    match name {
        "dpad.up"=>con.dpad.up_key,
        "dpad.down"=>con.dpad.down_key,
        "dpad.left"=>con.dpad.left_key,
        "dpad.right"=>con.dpad.right_key,
        "btn.circle"=>con.btns.circle,
        "btn.cross"=>con.btns.cross,
        "btn.cube"=>con.btns.cube,
        "btn.triangle"=>con.btns.triangle,
        "btn.l1"=>con.btns.l1,
        "btn.l2"=>con.btns.l2,
        "btn.r1"=>con.btns.r1,
        "btn.r2"=>con.btns.r2,
        _=>false
    }
}