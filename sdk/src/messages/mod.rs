/* Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

pub mod flush_unsaved_buffer;
mod partitioning;
mod partitioning_kind;
pub mod poll_messages;
mod polling_kind;
mod polling_strategy;
pub mod send_messages;

const MAX_HEADERS_SIZE: u32 = 100 * 1000;
pub const MAX_PAYLOAD_SIZE: u32 = 10 * 1000 * 1000;
pub use flush_unsaved_buffer::FlushUnsavedBuffer;
pub use partitioning::Partitioning;
pub use partitioning_kind::PartitioningKind;
pub use poll_messages::PollMessages;
pub use polling_kind::PollingKind;
pub use polling_strategy::PollingStrategy;
pub use send_messages::SendMessages;
