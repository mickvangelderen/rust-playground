#include <iostream>

int main() {
  char value = -1;
  std::cout << static_cast<size_t>(value) << std::endl;
  std::cout << static_cast<size_t>(static_cast<unsigned char>(value)) << std::endl;
}
