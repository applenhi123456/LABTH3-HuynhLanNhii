package lap3_0905;

public class Product {
    private String description;
    private String productID;
    private double price;

    // constructors
    public Product() {
        // default constructor
    }

    public Product(String description, String productID, double price) {
        this.description = description;
        this.productID = productID;
        this.price = price;
    }

    // getters
    public String getDescription() {
        return description;
    }

    public String getProductID() {
        return productID;
    }

    public double getPrice() {
        return price;
    }

    // setters
    public void setDescription(String description) {
        this.description = description;
    }

    public void setProductID(String productID) {
        this.productID = productID;
    }

    public void setPrice(double price) {
        this.price = price;
    }

    // toString method
    @Override
    public String toString() {
        return "Product{" +
                "description='" + description + '\'' +
                ", productID='" + productID + '\'' +
                ", price=" + price +
                '}';
    }
}