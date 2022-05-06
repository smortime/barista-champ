-- Add migration script here
INSERT INTO drink_types (drink_type)
VALUES
	("Aeropress"),
    	("IcedCoffee"),
    	("V60"),
    	("Chemex"),
    	("Cappuccino"),
    	("Americano"),
    	("Cortado");

INSERT INTO roasters (id, name)
VALUES
	("b92af527-fc18-4a8e-b378-99dca6161177", "Mythical Coffee"),
	("9063619c-aea1-4d54-9ee2-8aba19345e00", "Onyx Coffee Lab"),
	("1b503e35-428f-4d9d-a821-7b64276900e6", "Sterling Coffee"),
	("351fc3b4-6fd5-4aa9-9a03-fa25e4a275b2", "Black Oak Coffee Roasters");

INSERT INTO regions (id, name)
VALUES
	("ea22ad78-3b73-4684-a2d5-565240a08437", "Blend"),
	("cb36ddc7-bfe5-4091-a766-53dec2e871e0", "Ethiopia"),
	("f9bb8005-cce7-48e4-a7e6-88a9f803e4cd", "Columbia"),
	("284c84df-bb46-4344-811e-0ad6dea28a66", "Nicaragua"),
	("f7cea703-3d52-43b3-b37a-ae666cbef91e", "Guatemala");

INSERT INTO coffees (id, name, tasting_notes, style, region_id, roaster_id)
VALUES
	("f59b9499-4b6b-4fc9-811a-e11843db037e", "Thirteenth Sun", "Berry, Stone Fruit, Floral", "filtered", "cb36ddc7-bfe5-4091-a766-53dec2e871e0","351fc3b4-6fd5-4aa9-9a03-fa25e4a275b2"),
	("2a219898-1ce0-49d0-9ce3-64d0ab375eb0", "Bigfoot", "Chocolate covered cherry, green apple, mandarin orange", "filtered", "284c84df-bb46-4344-811e-0ad6dea28a66",  "b92af527-fc18-4a8e-b378-99dca6161177"),
	("704b8461-57b3-4b13-aedc-0c4dae9257c1", "Aponte Village", "Cherry, Floral Honey, Chocolate Malt, Papaya", "filtered", "f9bb8005-cce7-48e4-a7e6-88a9f803e4cd", "9063619c-aea1-4d54-9ee2-8aba19345e00"),
	("4d61e2f7-d681-4a8c-bca8-ccfacceb1d36", "Blendo Stupendo", "Cocoa, Caramel", "espresso", "ea22ad78-3b73-4684-a2d5-565240a08437", "1b503e35-428f-4d9d-a821-7b64276900e6");

INSERT INTO customers (id, name)
VALUES
	("3476a111-4f48-43cd-8535-f8d568333027", "Kristi"),
	("e4373e59-05f0-4e20-87f7-b2578243f31a", "Schuyler");

INSERT INTO orders (id, coffee_id, drink_type, customer_id)
VALUES
	("e637127a-155f-427e-8449-342b6ceea53f", "704b8461-57b3-4b13-aedc-0c4dae9257c1", "V60", "3476a111-4f48-43cd-8535-f8d568333027"),
	("f01b53b0-24da-41a3-b8ab-f83614f57549", "4d61e2f7-d681-4a8c-bca8-ccfacceb1d36", "Cortado", "e4373e59-05f0-4e20-87f7-b2578243f31a");



