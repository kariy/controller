import { Icon, useStyleConfig } from "@chakra-ui/react";

const KeyNew = (props: any) => {
  const { variant, size, ...rest } = props;
  const styles = useStyleConfig("Icon", { variant, size });

  return (
    <Icon viewBox="0 0 25 25" fill="currentColor" __css={styles} {...rest}>
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M10.0737 10.9831C10.608 10.8421 11.058 10.459 11.2759 9.93223C11.5539 9.26098 11.4002 8.48732 10.8863 7.97341C10.3725 7.45949 9.59937 7.30575 8.92772 7.58379C8.2568 7.86249 7.8188 8.51752 7.8188 9.24493C7.8188 9.72106 8.00831 10.1788 8.34523 10.5158C8.68213 10.8528 9.13911 11.0424 9.61582 11.0424C9.61582 11.0424 9.79325 11.0424 9.86147 11.0424C9.92968 11.0424 10.0737 10.9831 10.0737 10.9831ZM8.8804 9.25669C8.87992 9.06162 8.95732 8.87435 9.09542 8.73568C9.22117 8.60996 9.38769 8.53411 9.56371 8.52156H9.61576C9.913 8.52156 10.1811 8.70062 10.2951 8.97561C10.409 9.25041 10.3459 9.56705 10.1356 9.77735C9.92542 9.98762 9.60894 10.0508 9.33431 9.93686C9.05946 9.82286 8.8804 9.55412 8.8804 9.25669Z"
      />
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M15.4241 17.8181H16.068V18.589C16.0688 18.8238 16.1625 19.0494 16.3285 19.2155L16.8776 19.7647L16.881 19.768C17.0346 19.915 17.2383 19.9983 17.4516 20H19.6344C20.041 19.9991 20.3694 19.6694 20.3711 19.264V17.0795C20.3686 16.8599 20.2819 16.6498 20.1272 16.4925L15.6025 11.9726C16.1123 10.4953 15.9393 8.86455 15.1249 7.52516C14.2864 6.14675 12.8613 5.2293 11.259 5.03718C9.65739 4.8457 8.0558 5.40072 6.91569 6.54235C5.77301 7.68282 5.21675 9.28488 5.4082 10.8884C5.59964 12.4918 6.51681 13.9178 7.89605 14.7568C8.46801 15.1049 9.09315 15.3358 9.73674 15.4463C10.6008 15.5948 11.498 15.5263 12.3445 15.2333L13.0186 15.9076C13.1844 16.0749 13.4098 16.1687 13.6454 16.1687H14.4185V16.8128C14.4186 16.9603 14.451 17.1041 14.5117 17.2351C14.5609 17.3413 14.6287 17.4391 14.713 17.5234C14.9016 17.712 15.1576 17.8181 15.4241 17.8181ZM7.66702 7.29368L7.64855 7.31217C6.73836 8.23595 6.30659 9.52861 6.47895 10.815C6.65305 12.1102 7.42012 13.2498 8.55411 13.8982C9.68745 14.5466 11.0586 14.6296 12.2627 14.1229C12.4624 14.0388 12.6913 14.0851 12.8437 14.2365L13.2124 14.6028H13.2093L13.7113 15.105H14.804C15.1722 15.105 15.4708 15.4038 15.4708 15.772V16.7438H16.4517C16.8199 16.7438 17.1185 17.0427 17.1185 17.4108V18.5033L17.5531 18.9381H19.3076V17.1719L14.6098 12.4726C14.4567 12.3195 14.4122 12.0903 14.4949 11.8918C15.0032 10.6872 14.9211 9.31355 14.2723 8.17855C13.6243 7.04368 12.4838 6.27641 11.1885 6.10284C9.89325 5.92926 8.59103 6.36938 7.66702 7.29368Z"
      />
    </Icon>
  );
};

export default KeyNew;