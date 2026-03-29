# dinhthihlan

<img width="1700" height="257" alt="image" src="https://github.com/user-attachments/assets/d82976a5-f49f-4797-9e81-c57f3b9d1742" />

TÊN DỰ ÁN: StellarLibrary (Smart Contract)

VẤN ĐỀ (1 câu):

Sinh viên khó mua/thuê tài liệu học tập chất lượng theo cách nhanh, rẻ và có hệ thống, trong khi người soạn tài liệu khó bảo vệ công sức và nhận thù lao xứng đáng.

GIẢI PHÁP (1 câu):

Smart contract StellarLibrary trên Soroban tự động xử lý giao dịch “trả phí → cấp quyền truy cập” bằng cách nhận thanh toán bằng token (mô phỏng USDC trên testnet) và chuyển “Content Token” cho sinh viên để mở khóa tài liệu ngay lập tức.

TÍNH NĂNG STELLAR SỬ DỤNG:

[x] Chuyển XLM/USDC    [x] Token tùy chỉnh    [x] Soroban contract  

[ ] DEX tích hợp        [ ] Trustline          [ ] Clawback/Tuân thủ  

NGƯỜI DÙNG MỤC TIÊU:

Sinh viên (người mua/thuê tài liệu) và người soạn tài liệu/giáo trình (người bán nội dung).

TÍNH NĂNG CỐT LÕI (MVP):

Giao dịch duy nhất “Instant Access Unlock”:

- Sinh viên gọi hàm `unlock(student, creator)` trên contract.
- Contract chuyển đúng số tiền `price` (ví dụ 0.5 “USDC”) từ ví sinh viên sang ví creator (thông qua token transfer; sinh viên cần approve trước).
- Sau đó contract chuyển 1 “Content Token” cho sinh viên (đại diện quyền truy cập), để ứng dụng có thể kiểm tra balance/ownership và mở khóa file PDF tương ứng.

TẠI SAO STELLAR:

- Phí giao dịch siêu nhỏ: bán/thuê tài liệu giá dưới 1 USD thường không hiệu quả với cổng thanh toán truyền thống (phí cố định cao), trong khi Stellar/Soroban cho phép phí rất thấp nên micropayment khả thi.
- Tốc độ & trải nghiệm: giao dịch nhanh giúp “thanh toán xong mở khóa ngay”, phù hợp luồng mua nội dung tức thì.
- On-chain atomicity: thanh toán và cấp quyền có thể nằm trong cùng một luồng logic của contract, giảm rủi ro “đã trả tiền nhưng chưa được cấp quyền” (hoặc ngược lại) trong triển khai MVP.
