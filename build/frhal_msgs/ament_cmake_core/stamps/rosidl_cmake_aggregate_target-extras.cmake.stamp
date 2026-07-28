# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target frhal_msgs::frhal_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${frhal_msgs_TARGETS}.
if(frhal_msgs_TARGETS AND NOT TARGET frhal_msgs::frhal_msgs)
  add_library(frhal_msgs::frhal_msgs INTERFACE IMPORTED)
  set_target_properties(frhal_msgs::frhal_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${frhal_msgs_TARGETS}")
endif()
