import { memo } from "react";
import { Icon, useStyleConfig } from "@chakra-ui/react";
import { Props } from "../types";

export const DetailsLineIcon = memo(
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
          d="M8.267 6.133c0-.59.476-1.066 1.066-1.066h8.534c.59 0 1.066.476 1.066 1.066v5.334h-2.666a1.6 1.6 0 0 0-1.6 1.6v2.666H9.333c-.59 0-1.066-.476-1.066-1.066V6.133Zm10.606 6.4c-.053.15-.14.29-.253.404l-2.483 2.483a1.09 1.09 0 0 1-.404.253v-2.606c0-.294.24-.534.534-.534h2.606ZM7.2 6.133v8.534c0 1.176.957 2.133 2.133 2.133h6.05c.567 0 1.11-.223 1.51-.623l2.484-2.484c.4-.4.623-.943.623-1.51v-6.05A2.135 2.135 0 0 0 17.867 4H9.333A2.135 2.135 0 0 0 7.2 6.133Zm7.467 13.334a.535.535 0 0 0-.534-.534H8.267a3.2 3.2 0 0 1-3.2-3.2v-8a.535.535 0 0 0-.534-.533.535.535 0 0 0-.533.533v8A4.266 4.266 0 0 0 8.267 20h5.866c.294 0 .534-.24.534-.533Z"
        />
      </Icon>
    );
  },
);