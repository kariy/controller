import { Container, Footer } from "components/layout";
import { BigNumberish } from "starknet";
import { Policy } from "@cartridge/controller";
import { Button } from "@chakra-ui/react";
import { useState } from "react";
import { useConnection } from "hooks/connection";

export function CreateSession({
  onConnect,
  onCancel,
}: {
  onConnect: (policies: Policy[]) => void;
  onCancel: () => void;
}) {
  const { controller, policies, origin } = useConnection();
  const [isConnecting, setIsConnecting] = useState(false);
  const [expiresAt] = useState<bigint>(3000000000n);
  const [maxFees] = useState<BigNumberish>();
  return (
    <Container
      variant="connect"
      title="Create Session"
      description={`${origin} is requesting to connect to your Cartridge Controller`}
    >
      <Footer createSession>
        <Button
          colorScheme="colorful"
          isDisabled={isConnecting}
          isLoading={isConnecting}
          onClick={async () => {
            setIsConnecting(true);
            await controller
              .approve(origin, expiresAt, policies, maxFees)
              .then(() => {
                onConnect(policies);
              })
              .catch(() => {
                setIsConnecting(false);
              });
          }}
        >
          create
        </Button>

        <Button onClick={onCancel}>cancel</Button>
      </Footer>
    </Container>
  );
}
