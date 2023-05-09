import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        QuanLyHangHoa quanLyHangHoa = new QuanLyHangHoa();

        Scanner sc = new Scanner(System.in);

        while (true) {
            System.out.println("Menu:");
            System.out.println("1. Them hang hoa");
            System.out.println("2. Xuat danh sach hang hoa");
            System.out.println("3. Tim kiem hang hoa theo ma");
            System.out.println("4. Sua thong tin hang hoa");
            System.out.println("5. Xoa hang hoa");
            System.out.println("6. Thoat");

            System.out.print("Nhap lua chon cua ban: ");
            int luaChon = sc.nextInt();

            switch (luaChon) {
                case 1:
                    quanLyHangHoa.themHangHoa();
                    break;
                case 2:
                    quanLyHangHoa.xuatDanhSachHangHoa();
                    break;
                case 3:
                    quanLyHangHoa.timKiemHangHoaTheoMa();
                    break;
                case 4:
                    quanLyHangHoa.suaThongTinHangHoa();
                    break;
                case 5:
                    quanLyHangHoa.xoaHangHoa();
                    break;
                case 6:
                    System.exit(0);
                    break;
                default:
                    System.out.println("Lua chon khong hop le");
                    break;
            }
        }
    }
}

