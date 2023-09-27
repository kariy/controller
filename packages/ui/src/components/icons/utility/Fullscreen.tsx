import { memo } from "react";
import { Icon, IconProps } from "@chakra-ui/react";

export const FullscreenIcon = memo((props: IconProps) => (
  <Icon viewBox="0 0 24 24" {...props}>
    <path
      d="M4.5706 4.6687C4.2556 4.66927 4 4.90359 4 5.19232C4 5.2181 4.00187 5.24331 4.00625 5.26851L4.00562 5.26565V9.39275C4.00562 9.68207 4.26187 9.91637 4.57684 9.91637C4.89182 9.91637 5.14806 9.68205 5.14806 9.39275V6.4618L8.74164 9.75592C8.84539 9.85101 8.98789 9.90888 9.14537 9.90888C9.46098 9.90888 9.71658 9.67399 9.71658 9.38526C9.71658 9.24032 9.65284 9.10971 9.54909 9.01518L5.95551 5.72107H9.15355C9.46917 5.72107 9.72477 5.48618 9.72477 5.19745C9.72477 4.90872 9.46915 4.67383 9.15355 4.67383H4.64631C4.62381 4.6704 4.59747 4.6687 4.5706 4.6687ZM19.413 4.66927C19.3912 4.66985 19.3699 4.67157 19.3493 4.67443L19.3524 4.67386H14.8471C14.5315 4.67386 14.2759 4.90817 14.2759 5.19747C14.2759 5.48679 14.5315 5.72109 14.8471 5.72109H18.0451L14.4515 9.01521C14.3478 9.11031 14.284 9.24093 14.284 9.38529C14.284 9.6746 14.5403 9.9089 14.8553 9.9089C15.0128 9.9089 15.1559 9.85047 15.259 9.75594L18.8526 6.46182V9.39278C18.8526 9.68209 19.1082 9.9164 19.4238 9.9164C19.7394 9.9164 19.995 9.68208 19.995 9.39278V5.26171C19.9981 5.24109 20 5.21703 20 5.19239C20 4.90308 19.7444 4.66877 19.4288 4.66877C19.4232 4.66877 19.4187 4.6687 19.413 4.66927ZM4.5682 14.0892C4.25634 14.0938 4.0051 14.3264 4.0051 14.6128V18.8104C4.0051 18.8253 4.00635 18.8408 4.0076 18.8551L4.00698 18.8528C4.0076 18.8568 4.0076 18.8591 4.0076 18.8614V18.8597C4.00885 18.8751 4.01135 18.8883 4.01385 18.9021L4.01323 18.8986C4.01448 18.9067 4.01573 18.9118 4.01698 18.9164L4.01635 18.913C4.02823 18.9639 4.04698 19.0092 4.07072 19.0505L4.06947 19.0476C4.07322 19.0545 4.07572 19.0579 4.07822 19.0619L4.07697 19.059C4.08572 19.0728 4.0926 19.0837 4.10072 19.0946L4.09947 19.0928C4.10322 19.0986 4.10572 19.1014 4.10822 19.1049L4.10697 19.1032C4.13885 19.145 4.17572 19.1811 4.21697 19.212L4.21822 19.2132C4.21947 19.2143 4.22134 19.216 4.22322 19.2172C4.31634 19.2848 4.43508 19.3266 4.5632 19.3294C4.56633 19.33 4.57133 19.33 4.57633 19.33H9.15366C9.46927 19.33 9.72488 19.0957 9.72488 18.8064C9.72488 18.5171 9.46926 18.2828 9.15366 18.2828H5.95627L9.54985 14.9887C9.65609 14.8936 9.72234 14.7607 9.72234 14.614C9.72234 14.3247 9.46672 14.0904 9.15112 14.0904C9.14549 14.0904 9.13987 14.0904 9.13424 14.091C8.9805 14.095 8.84301 14.1545 8.74302 14.2485L5.14944 17.5426V14.6116C5.14944 14.3223 4.8932 14.088 4.57822 14.088H4.56947L4.5682 14.0892ZM19.4162 14.0892C19.1044 14.0938 18.8532 14.3264 18.8532 14.6128V17.5444L15.2596 14.2502C15.1558 14.1523 15.0108 14.0916 14.8502 14.0916C14.5352 14.0921 14.279 14.3264 14.279 14.6152C14.279 14.7624 14.3452 14.8953 14.4521 14.9898L18.0457 18.284H14.8483C14.5327 18.284 14.2771 18.5183 14.2771 18.8076C14.2771 19.0969 14.5327 19.3312 14.8483 19.3312H19.425C19.485 19.3312 19.5437 19.3226 19.5981 19.3066L19.5943 19.3077C19.5981 19.3066 19.5975 19.3066 19.5981 19.3066L19.5956 19.3071C19.6168 19.3014 19.6343 19.2951 19.6518 19.2882L19.6481 19.2894C19.6524 19.2877 19.6537 19.2871 19.6543 19.2865L19.6518 19.2882C19.6712 19.2802 19.6868 19.2728 19.7024 19.2647L19.6993 19.2659C19.7018 19.2647 19.7018 19.2647 19.7024 19.2647C19.7318 19.2498 19.7562 19.2344 19.7799 19.2178L19.7787 19.2189C19.7874 19.2132 19.7937 19.208 19.8006 19.2023L19.7993 19.2029C19.8124 19.1926 19.8237 19.1828 19.8349 19.1719C19.8356 19.1714 19.8362 19.1708 19.8362 19.1708C19.8612 19.1467 19.8843 19.1209 19.9037 19.0923L19.9049 19.09C19.9037 19.0929 19.9037 19.0923 19.9037 19.0923L19.9049 19.09C19.9137 19.078 19.9231 19.0631 19.9312 19.0476L19.9324 19.0448C19.9312 19.0476 19.9312 19.0476 19.9312 19.0476L19.9324 19.0448C19.9481 19.0173 19.9618 18.9846 19.9718 18.9514L19.9731 18.9479C19.9724 18.9508 19.9731 18.9491 19.9731 18.9479L19.9737 18.9456C19.9849 18.9078 19.9924 18.8654 19.9931 18.8208C19.9937 18.8179 19.9937 18.8133 19.9937 18.8087V14.6134C19.9937 14.3241 19.7381 14.0898 19.4225 14.0898H19.4137L19.4162 14.0892Z"
      fill="currentColor"
    />
  </Icon>
));
