import { PropsWithChildren } from "react";
import { StyleSheet, View } from "react-native";

export function AppContainer({ children }: PropsWithChildren) {
  return (
    <View style={styles.container}>
      <View style={styles.content}>{children}</View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
  },
  content: {
    flex: 1,
    width: "100%",
    maxWidth: 800,
  },
});
