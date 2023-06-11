import { memo } from "react";
import { Icon, useStyleConfig } from "@chakra-ui/react";
import { Props } from "../types";

export const FilterSolidIcon = memo(
  ({
    variant,
    size,
    boxSize = 6,
    colorScheme,
    orientation,
    styleConfig,
    ...iconProps
  }: Props) => {
    const styles = useStyleConfig("Icon", {
      variant,
      size,
      colorScheme,
      orientation,
      styleConfig,
    });

    return (
      <Icon viewBox="0 0 24 24" __css={styles} boxSize={boxSize} {...iconProps}>
        <path
          fill="currentColor"
          d="M4.122 5.715A1.245 1.245 0 0 1 5.25 5h13.502a1.253 1.253 0 0 1 .966 2.044L14 14.029V18a.998.998 0 0 1-1.6.8l-2-1.5a.993.993 0 0 1-.4-.8v-2.472L4.28 7.04a1.247 1.247 0 0 1-.16-1.325Z"
        />
      </Icon>
    );
  },
);